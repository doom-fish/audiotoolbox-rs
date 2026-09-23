//! Executor-agnostic async wrappers for AudioToolbox callback surfaces.
//!
//! Enable this module with the `async` Cargo feature:
//!
//! ```toml
//! [dependencies]
//! audiotoolbox = { version = "0.5", features = ["async"] }
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
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::spsc::{PopFuture, SpscConsumer, SpscProducer, SpscRing};

use crate::{
    AUGraph, AudioBufferList1, AudioTimeStamp, AudioToolboxError, AudioUnit, AudioUnitElement,
    AudioUnitPropertyId, AudioUnitRenderActionFlags, AudioUnitScope, OSStatus, Result, NO_ERR,
};

const STREAM_MAX_CAPACITY: usize = 4096;

type EventConsumer<T> = SpscConsumer<T, STREAM_MAX_CAPACITY>;
type EventNext<'a, T> = PopFuture<'a, T, STREAM_MAX_CAPACITY>;
type EventContext<T> = CallbackContext<DetachableProducer<T>>;

fn invalid_capacity(operation: &'static str) -> AudioToolboxError {
    AudioToolboxError::message(operation, "async stream capacity must be > 0")
}

struct DetachableProducer<T> {
    producer: AtomicPtr<SpscProducer<T, STREAM_MAX_CAPACITY>>,
    in_flight: AtomicUsize,
}

impl<T> DetachableProducer<T> {
    fn new(producer: SpscProducer<T, STREAM_MAX_CAPACITY>) -> Self {
        Self {
            producer: AtomicPtr::new(Box::into_raw(Box::new(producer))),
            in_flight: AtomicUsize::new(0),
        }
    }

    fn push(&self, item: T) {
        self.in_flight.fetch_add(1, Ordering::SeqCst);
        let producer = self.producer.load(Ordering::SeqCst);
        if let Some(producer) = unsafe { producer.as_ref() } {
            let _ = producer.push_overwrite(item);
        }
        self.in_flight.fetch_sub(1, Ordering::SeqCst);
    }

    fn detach(&self) {
        let producer = self.producer.swap(core::ptr::null_mut(), Ordering::SeqCst);
        while self.in_flight.load(Ordering::SeqCst) != 0 {
            std::thread::yield_now();
        }
        if !producer.is_null() {
            drop(unsafe { Box::from_raw(producer) });
        }
    }

    fn is_attached(&self) -> bool {
        !self.producer.load(Ordering::SeqCst).is_null()
    }
}

impl<T> Drop for DetachableProducer<T> {
    fn drop(&mut self) {
        let producer = *self.producer.get_mut();
        if !producer.is_null() {
            drop(unsafe { Box::from_raw(producer) });
        }
    }
}

fn event_channel<T: Send + 'static>(
    capacity: usize,
    operation: &'static str,
) -> Result<(EventConsumer<T>, EventContext<T>)> {
    if capacity == 0 {
        return Err(invalid_capacity(operation));
    }
    let (producer, consumer) =
        SpscRing::<T, STREAM_MAX_CAPACITY>::with_capacity(capacity.min(STREAM_MAX_CAPACITY));
    Ok((
        consumer,
        CallbackContext::new(DetachableProducer::new(producer)),
    ))
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
    let _ = unsafe {
        EventContext::<AudioUnitPropertyEvent>::with(
            user_data,
            "audiotoolbox::audio_unit_property_listener_cb",
            |slot| {
                slot.push(AudioUnitPropertyEvent {
                    property_id,
                    scope,
                    element,
                });
            },
        )
    };
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
) -> OSStatus {
    let _ = unsafe {
        EventContext::<RenderNotifyEvent>::with(
            user_data,
            "audiotoolbox::render_notify_cb",
            |slot| {
                slot.push(render_notify_event_from_raw(
                    action_flags,
                    time_stamp,
                    bus_number,
                    number_frames,
                ));
            },
        )
    };
    NO_ERR
}

/// Async stream of `AudioUnit` property-listener callbacks.
pub struct AudioUnitPropertyStream {
    inner: EventConsumer<AudioUnitPropertyEvent>,
    property_id: AudioUnitPropertyId,
    context: EventContext<AudioUnitPropertyEvent>,
    unit: AudioUnit,
}

impl Drop for AudioUnitPropertyStream {
    fn drop(&mut self) {
        let _ = unsafe {
            self.unit.remove_property_listener_with_user_data(
                self.property_id,
                audio_unit_property_listener_cb,
                self.context.as_ptr(),
            )
        };
        self.context.get().detach();
    }
}

impl AudioUnitPropertyStream {
    /// Subscribe to callbacks from `AudioUnitAddPropertyListener`.
    pub fn subscribe(
        unit: &AudioUnit,
        property_id: AudioUnitPropertyId,
        capacity: usize,
    ) -> Result<Self> {
        let (inner, context) = event_channel(capacity, "AudioUnitPropertyStream::subscribe")?;
        let unit = unit.retained()?;

        unsafe {
            unit.add_property_listener(
                property_id,
                audio_unit_property_listener_cb,
                context.as_ptr(),
            )
        }?;
        unit.adopt_context(
            context.retained_ptr(),
            EventContext::<AudioUnitPropertyEvent>::RELEASE,
        );

        Ok(Self {
            inner,
            property_id,
            context,
            unit,
        })
    }

    pub const fn next(&self) -> EventNext<'_, AudioUnitPropertyEvent> {
        self.inner.pop_async()
    }

    pub fn try_next(&self) -> Option<AudioUnitPropertyEvent> {
        self.inner.pop()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    pub fn is_closed(&self) -> bool {
        !self.context.get().is_attached()
    }
}

/// Async stream of `AudioUnitAddRenderNotify` callbacks.
pub struct AudioUnitRenderNotifyStream {
    inner: EventConsumer<RenderNotifyEvent>,
    context: EventContext<RenderNotifyEvent>,
    unit: AudioUnit,
}

impl Drop for AudioUnitRenderNotifyStream {
    fn drop(&mut self) {
        let _ = unsafe {
            self.unit
                .remove_render_notify(render_notify_cb, self.context.as_ptr())
        };
        self.context.get().detach();
    }
}

impl AudioUnitRenderNotifyStream {
    /// Subscribe to `AudioUnitAddRenderNotify` using a real-time-safe SPSC handoff.
    pub fn subscribe(unit: &AudioUnit, capacity: usize) -> Result<Self> {
        let (inner, context) = event_channel(capacity, "AudioUnitRenderNotifyStream::subscribe")?;
        let unit = unit.retained()?;

        unsafe { unit.add_render_notify(render_notify_cb, context.as_ptr()) }?;
        unit.adopt_context(
            context.retained_ptr(),
            EventContext::<RenderNotifyEvent>::RELEASE,
        );

        Ok(Self {
            inner,
            context,
            unit,
        })
    }

    pub const fn next(&self) -> EventNext<'_, RenderNotifyEvent> {
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
    inner: EventConsumer<RenderNotifyEvent>,
    context: EventContext<RenderNotifyEvent>,
    graph: AUGraph,
}

impl Drop for AUGraphRenderNotifyStream {
    fn drop(&mut self) {
        let _ = unsafe {
            self.graph
                .remove_render_notify(render_notify_cb, self.context.as_ptr())
        };
        self.context.get().detach();
    }
}

impl AUGraphRenderNotifyStream {
    /// Subscribe to `AUGraphAddRenderNotify` using a real-time-safe SPSC handoff.
    pub fn subscribe(graph: &AUGraph, capacity: usize) -> Result<Self> {
        let (inner, context) = event_channel(capacity, "AUGraphRenderNotifyStream::subscribe")?;
        let graph = graph.retained()?;

        unsafe { graph.add_render_notify(render_notify_cb, context.as_ptr()) }?;
        graph.adopt_context(
            context.retained_ptr(),
            EventContext::<RenderNotifyEvent>::RELEASE,
        );

        Ok(Self {
            inner,
            context,
            graph,
        })
    }

    pub const fn next(&self) -> EventNext<'_, RenderNotifyEvent> {
        self.inner.pop_async()
    }

    pub fn try_next(&self) -> Option<RenderNotifyEvent> {
        self.inner.pop()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::thread;

    use doom_fish_utils::spsc::SpscRing;

    use super::{DetachableProducer, STREAM_MAX_CAPACITY};

    #[test]
    fn detach_frees_the_producer_and_later_pushes_are_dropped() {
        let (producer, consumer) = SpscRing::<u32, STREAM_MAX_CAPACITY>::with_capacity(4);
        let slot = DetachableProducer::new(producer);
        slot.push(1);
        slot.push(2);
        assert!(slot.is_attached());

        slot.detach();
        assert!(!slot.is_attached());
        assert!(consumer.is_closed());
        slot.push(3);

        assert_eq!(consumer.pop(), Some(1));
        assert_eq!(consumer.pop(), Some(2));
        assert_eq!(consumer.pop(), None);
    }

    #[test]
    fn full_ring_drops_the_oldest_event() {
        let (producer, consumer) = SpscRing::<u32, STREAM_MAX_CAPACITY>::with_capacity(2);
        let slot = DetachableProducer::new(producer);
        for value in 0..5 {
            slot.push(value);
        }
        assert_eq!(consumer.pop(), Some(3));
        assert_eq!(consumer.pop(), Some(4));
        assert_eq!(consumer.pop(), None);
    }

    #[test]
    fn detach_waits_for_a_concurrent_pusher() {
        for _ in 0..200 {
            let (producer, consumer) = SpscRing::<u64, STREAM_MAX_CAPACITY>::with_capacity(64);
            let slot = Arc::new(DetachableProducer::new(producer));
            let running = Arc::new(AtomicBool::new(true));
            let pusher = {
                let slot = Arc::clone(&slot);
                let running = Arc::clone(&running);
                thread::spawn(move || {
                    let mut value = 0_u64;
                    while running.load(Ordering::Relaxed) {
                        slot.push(value);
                        value += 1;
                    }
                    value
                })
            };
            while consumer.buffered_count() == 0 {
                std::hint::spin_loop();
            }
            slot.detach();
            assert!(consumer.is_closed());
            running.store(false, Ordering::Relaxed);
            let pushed = pusher.join().expect("pusher thread");
            assert!(pushed > 0);
            let mut drained = 0;
            while consumer.pop().is_some() {
                drained += 1;
            }
            assert!(drained > 0);
        }
    }
}
