#![allow(dead_code)]

use std::ffi::c_void;

use audiotoolbox::{
    AURenderCallbackStruct, AudioBufferList1, AudioStreamBasicDescription, AudioTimeStamp,
    AudioUnit, AudioUnitRenderActionFlags, OSStatus, Result, SMPTETime,
    AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER, AUDIO_UNIT_PROPERTY_SET_RENDER_CALLBACK,
    AUDIO_UNIT_SCOPE_INPUT, AUDIO_UNIT_SCOPE_OUTPUT, AUDIO_UNIT_SUBTYPE_AU_CONVERTER, NO_ERR,
};

pub const RAMP_STEP: i16 = 64;

unsafe extern "C" fn ramp_input(
    _ref_con: *mut c_void,
    _flags: *mut AudioUnitRenderActionFlags,
    _time_stamp: *const AudioTimeStamp,
    _bus: u32,
    frames: u32,
    io_data: *mut AudioBufferList1,
) -> OSStatus {
    let Some(list) = (unsafe { io_data.as_mut() }) else {
        return -50;
    };
    let buffer = &mut list.mBuffers[0];
    if buffer.mData.is_null() || (buffer.mDataByteSize as usize) < frames as usize * 2 {
        return -50;
    }
    let samples =
        unsafe { std::slice::from_raw_parts_mut(buffer.mData.cast::<i16>(), frames as usize) };
    for (index, sample) in samples.iter_mut().enumerate() {
        *sample = i16::try_from(index % 512).unwrap_or(0) * RAMP_STEP;
    }
    NO_ERR
}

pub fn ramp_converter_unit() -> Result<(AudioUnit, AudioStreamBasicDescription)> {
    let unit = AudioUnit::new_apple(
        AUDIO_COMPONENT_TYPE_FORMAT_CONVERTER,
        AUDIO_UNIT_SUBTYPE_AU_CONVERTER,
    )?;
    let input = AudioStreamBasicDescription::linear_pcm_i16(44_100.0, 1, true);
    let output = AudioStreamBasicDescription::linear_pcm_f32(44_100.0, 1, true);
    unit.set_stream_format(AUDIO_UNIT_SCOPE_INPUT, 0, &input)?;
    unit.set_stream_format(AUDIO_UNIT_SCOPE_OUTPUT, 0, &output)?;
    let callback = AURenderCallbackStruct {
        inputProc: Some(ramp_input),
        inputProcRefCon: std::ptr::null_mut(),
    };
    unsafe {
        unit.set_property_typed(
            AUDIO_UNIT_PROPERTY_SET_RENDER_CALLBACK,
            AUDIO_UNIT_SCOPE_INPUT,
            0,
            &callback,
            "AudioUnitSetProperty(render callback)",
        )
    }?;
    unit.initialize()?;
    Ok((unit, output))
}

pub fn sample_time_stamp(sample_time: f64) -> AudioTimeStamp {
    AudioTimeStamp {
        mSampleTime: sample_time,
        mHostTime: 0,
        mRateScalar: 0.0,
        mWordClockTime: 0,
        mSMPTETime: SMPTETime {
            mSubframes: 0,
            mSubframeDivisor: 0,
            mCounter: 0,
            mType: 0,
            mFlags: 0,
            mHours: 0,
            mMinutes: 0,
            mSeconds: 0,
            mFrames: 0,
        },
        mFlags: 1,
        mReserved: 0,
    }
}
