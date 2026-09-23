#![cfg(feature = "async")]

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

use audiotoolbox::{
    AudioBuffer, AudioBufferList, AudioBufferList1, AudioTimeStamp, OSStatus, Result, NO_ERR,
};

mod support;

#[link(name = "AudioToolbox", kind = "framework")]
unsafe extern "C" {
    fn AudioUnitRender(
        unit: *mut c_void,
        io_action_flags: *mut u32,
        time_stamp: *const AudioTimeStamp,
        output_bus_number: u32,
        number_frames: u32,
        io_data: *mut AudioBufferList1,
    ) -> OSStatus;
}

#[test]
fn dropping_render_notify_streams_while_rendering_is_sound() -> Result<()> {
    let (unit, _) = support::ramp_converter_unit()?;
    let raw_unit = unit.as_raw() as usize;
    let running = Arc::new(AtomicBool::new(true));
    let renders = Arc::new(AtomicUsize::new(0));

    let render_thread = {
        let running = Arc::clone(&running);
        let renders = Arc::clone(&renders);
        thread::spawn(move || {
            let mut samples = vec![0_f32; 256];
            let mut sample_time = 0.0;
            while running.load(Ordering::Relaxed) {
                let mut flags = 0;
                let time_stamp = support::sample_time_stamp(sample_time);
                let mut list = AudioBufferList {
                    mNumberBuffers: 1,
                    mBuffers: [AudioBuffer {
                        mNumberChannels: 1,
                        mDataByteSize: 1024,
                        mData: samples.as_mut_ptr().cast(),
                    }],
                };
                let status = unsafe {
                    AudioUnitRender(
                        raw_unit as *mut c_void,
                        &raw mut flags,
                        &raw const time_stamp,
                        0,
                        256,
                        &raw mut list,
                    )
                };
                if status == NO_ERR {
                    renders.fetch_add(1, Ordering::Relaxed);
                }
                sample_time += 256.0;
            }
        })
    };

    let mut events = 0_usize;
    for _ in 0..2000 {
        let stream = unit.render_notify_stream(64)?;
        thread::yield_now();
        while let Some(event) = stream.try_next() {
            assert_eq!(event.number_frames, 256);
            events += 1;
        }
        drop(stream);
    }
    let property_streams = (0..50)
        .map(|_| unit.property_events(audiotoolbox::AUDIO_UNIT_PROPERTY_LAST_RENDER_ERROR, 8))
        .collect::<Result<Vec<_>>>()?;
    drop(property_streams);

    running.store(false, Ordering::Relaxed);
    render_thread.join().expect("render thread");
    assert!(renders.load(Ordering::Relaxed) > 0);
    assert!(events > 0, "no render-notify events were observed");
    Ok(())
}
