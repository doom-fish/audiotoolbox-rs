//! Executor-agnostic async wrappers for AudioToolbox callback surfaces.
//!
//! Enable this module with the `async` Cargo feature:
//!
//! ```toml
//! [dependencies]
//! audiotoolbox = { version = "0.4", features = ["async"] }
//! ```
//!
//! The wrappers here intentionally target callback surfaces that are naturally
//! asynchronous or event-like:
//!
//! - `AudioUnit` property listeners
//! - `AudioUnit` render-notify callbacks
//! - `AUGraph` render-notify callbacks
//!
//! Synchronous pull-render callbacks such as `AURenderCallbackStruct` /
//! `AUGraphSetNodeInputCallback` are deliberately left as synchronous APIs.
//!
//! # Example
//!
//! ```rust,no_run
//! use audiotoolbox::{
//!     AudioUnit, AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER, AUDIO_UNIT_PROPERTY_STREAM_FORMAT,
//!     AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
//! };
//!
//! # async fn run() -> Result<(), audiotoolbox::AudioToolboxError> {
//! let unit = AudioUnit::new_apple(
//!     AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
//!     AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
//! )?;
//!
//! let property_stream = unit.property_events(AUDIO_UNIT_PROPERTY_STREAM_FORMAT, 16)?;
//! let render_stream = unit.render_notify_stream(128)?;
//!
//! let _ = property_stream.buffered_count() + render_stream.buffered_count();
//! # Ok(())
//! # }
//! ```

#![cfg(feature = "async")]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::c_void;

use doom_fish_utils::panic_safe::catch_user_panic;
use doom_fish_utils::spsc::{PopFuture as SpscPopFuture, SpscConsumer, SpscProducer, SpscRing};
use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};

use crate::{
    AUGraph, AudioBufferList1, AudioTimeStamp, AudioToolboxError, AudioUnit, AudioUnitElement,
    AudioUnitPropertyId, AudioUnitRenderActionFlags, AudioUnitScope, Result, NO_ERR,
};

const RENDER_NOTIFY_STREAM_MAX_CAPACITY: usize = 4096;

type RenderNotifyProducer = SpscProducer<RenderNotifyEvent, RENDER_NOTIFY_STREAM_MAX_CAPACITY>;
type RenderNotifyConsumer = SpscConsumer<RenderNotifyEvent, RENDER_NOTIFY_STREAM_MAX_CAPACITY>;
type RenderNotifyNext<'a> = SpscPopFuture<'a, RenderNotifyEvent, RENDER_NOTIFY_STREAM_MAX_CAPACITY>;

fn invalid_capacity(operation: &'static str) -> AudioToolboxError {
    AudioToolboxError::message(operation, "async stream capacity must be > 0")
}

fn drop_boxed_ptr<T>(raw: &mut *mut T) {
    if !(*raw).is_null() {
        // SAFETY: `*raw` originates from `Box::into_raw` in this module and is
        // reconstituted at most once here before being nulled out.
        unsafe { drop(Box::from_raw(*raw)) };
        *raw = core::ptr::null_mut();
    }
}

/// One `AudioUnitAddPropertyListener` callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioUnitPropertyEvent {
    pub property_id: AudioUnitPropertyId,
    pub scope: AudioUnitScope,
    pub element: AudioUnitElement,
}

/// Snapshot of one render-notify callback.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderNotifyEvent {
    pub action_flags: AudioUnitRenderActionFlags,
    pub sample_time: f64,
    pub host_time: u64,
    pub bus_number: u32,
    pub number_frames: u32,
}

unsafe extern "C" fn audio_unit_property_listener_cb(
    user_data: *mut c_void,
    _audio_unit: *mut c_void,
    property_id: AudioUnitPropertyId,
    scope: AudioUnitScope,
    element: AudioUnitElement,
) {
    catch_user_panic("audiotoolbox::audio_unit_property_listener_cb", || {
        // SAFETY: `user_data` is the boxed sender pointer allocated in
        // `AudioUnitPropertyStream::subscribe` and remains valid until the
        // listener is removed during drop.
        let Some(sender) = (unsafe {
            user_data
                .cast::<AsyncStreamSender<AudioUnitPropertyEvent>>()
                .as_ref()
        }) else {
            return;
        };
        sender.push(AudioUnitPropertyEvent {
            property_id,
            scope,
            element,
        });
    });
}

unsafe fn render_notify_event_from_raw(
    action_flags: *mut AudioUnitRenderActionFlags,
    time_stamp: *const AudioTimeStamp,
    bus_number: u32,
    number_frames: u32,
) -> RenderNotifyEvent {
    // SAFETY: pointers are either null or valid for the duration of the render
    // notify callback. Null pointers are mapped to zero/default metadata.
    let action_flags = unsafe { action_flags.as_ref().copied() }.unwrap_or(0);
    // SAFETY: see comment above.
    let time_stamp = unsafe { time_stamp.as_ref() };

    RenderNotifyEvent {
        action_flags,
        sample_time: time_stamp.map_or(0.0, |time_stamp| time_stamp.mSampleTime),
        host_time: time_stamp.map_or(0, |time_stamp| time_stamp.mHostTime),
        bus_number,
        number_frames,
    }
}

unsafe extern "C" fn render_notify_cb(
    user_data: *mut c_void,
    action_flags: *mut AudioUnitRenderActionFlags,
    time_stamp: *const AudioTimeStamp,
    bus_number: u32,
    number_frames: u32,
    _io_data: *mut AudioBufferList1,
) -> i32 {
    catch_user_panic("audiotoolbox::render_notify_cb", || {
        // SAFETY: `user_data` is the boxed producer pointer allocated in the
        // corresponding `subscribe` constructor and stays valid until the notify
        // callback is removed during drop.
        let Some(sender) = (unsafe { user_data.cast::<RenderNotifyProducer>().as_ref() }) else {
            return;
        };
        // SAFETY: the callback receives valid pointers for the duration of this
        // invocation; `render_notify_event_from_raw` defensively handles nulls.
        let _ = sender.push_overwrite(unsafe {
            render_notify_event_from_raw(action_flags, time_stamp, bus_number, number_frames)
        });
    });
    NO_ERR
}

/// Async stream of `AudioUnit` property-listener callbacks.
pub struct AudioUnitPropertyStream {
    inner: BoundedAsyncStream<AudioUnitPropertyEvent>,
    property_id: AudioUnitPropertyId,
    sender_raw: *mut AsyncStreamSender<AudioUnitPropertyEvent>,
    unit: AudioUnit,
}

impl Drop for AudioUnitPropertyStream {
    fn drop(&mut self) {
        let _ = unsafe {
            self.unit.remove_property_listener_with_user_data(
                self.property_id,
                audio_unit_property_listener_cb,
                self.sender_raw.cast(),
            )
        };
        drop_boxed_ptr(&mut self.sender_raw);
    }
}

impl AudioUnitPropertyStream {
    /// Subscribe to callbacks from `AudioUnitAddPropertyListener`.
    pub fn subscribe(
        unit: &AudioUnit,
        property_id: AudioUnitPropertyId,
        capacity: usize,
    ) -> Result<Self> {
        if capacity == 0 {
            return Err(invalid_capacity("AudioUnitPropertyStream::subscribe"));
        }

        let (inner, sender) = BoundedAsyncStream::new(capacity);
        let mut sender_raw = Box::into_raw(Box::new(sender));
        let unit = unit.retained()?;

        if let Err(error) = unsafe {
            unit.add_property_listener(
                property_id,
                audio_unit_property_listener_cb,
                sender_raw.cast(),
            )
        } {
            drop_boxed_ptr(&mut sender_raw);
            return Err(error);
        }

        Ok(Self {
            inner,
            property_id,
            sender_raw,
            unit,
        })
    }

    pub const fn next(&self) -> NextItem<'_, AudioUnitPropertyEvent> {
        self.inner.next()
    }

    pub fn try_next(&self) -> Option<AudioUnitPropertyEvent> {
        self.inner.try_next()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

/// Async stream of `AudioUnitAddRenderNotify` callbacks.
pub struct AudioUnitRenderNotifyStream {
    inner: RenderNotifyConsumer,
    sender_raw: *mut RenderNotifyProducer,
    unit: AudioUnit,
}

impl Drop for AudioUnitRenderNotifyStream {
    fn drop(&mut self) {
        let _ = unsafe {
            self.unit
                .remove_render_notify(render_notify_cb, self.sender_raw.cast())
        };
        drop_boxed_ptr(&mut self.sender_raw);
    }
}

impl AudioUnitRenderNotifyStream {
    /// Subscribe to `AudioUnitAddRenderNotify` using a real-time-safe SPSC handoff.
    pub fn subscribe(unit: &AudioUnit, capacity: usize) -> Result<Self> {
        if capacity == 0 {
            return Err(invalid_capacity("AudioUnitRenderNotifyStream::subscribe"));
        }

        let ring_capacity = capacity.min(RENDER_NOTIFY_STREAM_MAX_CAPACITY);
        let (sender, inner) =
            SpscRing::<RenderNotifyEvent, RENDER_NOTIFY_STREAM_MAX_CAPACITY>::with_capacity(
                ring_capacity,
            );
        let mut sender_raw = Box::into_raw(Box::new(sender));
        let unit = unit.retained()?;

        if let Err(error) = unsafe { unit.add_render_notify(render_notify_cb, sender_raw.cast()) } {
            drop_boxed_ptr(&mut sender_raw);
            return Err(error);
        }

        Ok(Self {
            inner,
            sender_raw,
            unit,
        })
    }

    pub const fn next(&self) -> RenderNotifyNext<'_> {
        self.inner.pop_async()
    }

    pub fn try_next(&self) -> Option<RenderNotifyEvent> {
        self.inner.pop()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

/// Async stream of `AUGraphAddRenderNotify` callbacks.
pub struct AUGraphRenderNotifyStream {
    inner: RenderNotifyConsumer,
    sender_raw: *mut RenderNotifyProducer,
    graph: AUGraph,
}

impl Drop for AUGraphRenderNotifyStream {
    fn drop(&mut self) {
        let _ = unsafe {
            self.graph
                .remove_render_notify(render_notify_cb, self.sender_raw.cast())
        };
        drop_boxed_ptr(&mut self.sender_raw);
    }
}

impl AUGraphRenderNotifyStream {
    /// Subscribe to `AUGraphAddRenderNotify` using a real-time-safe SPSC handoff.
    pub fn subscribe(graph: &AUGraph, capacity: usize) -> Result<Self> {
        if capacity == 0 {
            return Err(invalid_capacity("AUGraphRenderNotifyStream::subscribe"));
        }

        let ring_capacity = capacity.min(RENDER_NOTIFY_STREAM_MAX_CAPACITY);
        let (sender, inner) =
            SpscRing::<RenderNotifyEvent, RENDER_NOTIFY_STREAM_MAX_CAPACITY>::with_capacity(
                ring_capacity,
            );
        let mut sender_raw = Box::into_raw(Box::new(sender));
        let graph = graph.retained()?;

        if let Err(error) = unsafe { graph.add_render_notify(render_notify_cb, sender_raw.cast()) }
        {
            drop_boxed_ptr(&mut sender_raw);
            return Err(error);
        }

        Ok(Self {
            inner,
            sender_raw,
            graph,
        })
    }

    pub const fn next(&self) -> RenderNotifyNext<'_> {
        self.inner.pop_async()
    }

    pub fn try_next(&self) -> Option<RenderNotifyEvent> {
        self.inner.pop()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}
