use std::ffi::c_void;
use std::mem::{size_of, MaybeUninit};

use crate::{
    internal::status_to_result, AVAudioConverterPrimeInfo, AVBeatRange, AudioBytePacketTranslation,
    AudioClassDescription, AudioComponentDescription, AudioConverterPrimeInfo,
    AudioFilePacketTableInfo, AudioFileTypeAndFormatId, AudioFormatListItem,
    AudioFramePacketTranslation, AudioIndependentPacketTranslation,
    AudioPacketDependencyInfoTranslation, AudioPacketRangeByteCountTranslation,
    AudioPacketRollDistanceTranslation, AudioStreamBasicDescription, AudioStreamPacketDescription,
    AudioTimeStamp, AudioToolboxError, AudioValueRange, CABarBeatTime, ExtendedControlEvent,
    ExtendedTempoEvent, MIDIChannelMessage, MIDINoteMessage, MusicDeviceStdNoteParams,
    MusicTrackLoopInfo, NoteParamsControlValue, OSStatus, ParameterEvent, Result, SMPTETime,
};

#[allow(clippy::missing_safety_doc)]
pub unsafe trait AudioProperty: Copy + 'static {}

macro_rules! audio_property {
    ($($ty:ty),+ $(,)?) => {
        $(unsafe impl AudioProperty for $ty {})+
    };
}

audio_property!(
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    f32,
    f64,
    AudioStreamBasicDescription,
    AudioStreamPacketDescription,
    AudioValueRange,
    AudioClassDescription,
    AudioComponentDescription,
    AudioFormatListItem,
    AudioTimeStamp,
    SMPTETime,
    AudioConverterPrimeInfo,
    AudioFilePacketTableInfo,
    AudioFileTypeAndFormatId,
    AudioFramePacketTranslation,
    AudioBytePacketTranslation,
    AudioPacketRangeByteCountTranslation,
    AudioPacketRollDistanceTranslation,
    AudioIndependentPacketTranslation,
    AudioPacketDependencyInfoTranslation,
    CABarBeatTime,
    MusicTrackLoopInfo,
    MIDINoteMessage,
    MIDIChannelMessage,
    ParameterEvent,
    ExtendedTempoEvent,
    ExtendedControlEvent,
    MusicDeviceStdNoteParams,
    NoteParamsControlValue,
    AVAudioConverterPrimeInfo,
    AVBeatRange,
);

unsafe impl<T: AudioProperty, const N: usize> AudioProperty for [T; N] {}

pub(crate) fn property_byte_size<T>(operation: &'static str) -> Result<u32> {
    u32::try_from(size_of::<T>()).map_err(|_| {
        AudioToolboxError::message(operation, "property type exceeds UInt32::MAX bytes")
    })
}

pub(crate) fn read_property<T: AudioProperty>(
    operation: &'static str,
    read: impl FnOnce(*mut c_void, *mut u32) -> OSStatus,
) -> Result<T> {
    let expected = property_byte_size::<T>(operation)?;
    let mut value = MaybeUninit::<T>::zeroed();
    let mut size = expected;
    status_to_result(operation, read(value.as_mut_ptr().cast(), &raw mut size))?;
    if size != expected {
        return Err(AudioToolboxError::message(
            operation,
            format!("property returned {size} bytes, expected {expected}"),
        ));
    }
    Ok(unsafe { value.assume_init() })
}

pub(crate) fn read_property_array<T: AudioProperty>(
    operation: &'static str,
    byte_size: u32,
    read: impl FnOnce(*mut c_void, *mut u32) -> OSStatus,
) -> Result<Vec<T>> {
    let element_size = size_of::<T>();
    if element_size == 0 {
        return Err(AudioToolboxError::message(
            operation,
            "property element type must not be zero-sized",
        ));
    }
    let capacity_bytes = byte_size as usize;
    if capacity_bytes % element_size != 0 {
        return Err(AudioToolboxError::message(
            operation,
            "property payload is not an integral number of elements",
        ));
    }
    if capacity_bytes == 0 {
        return Ok(Vec::new());
    }
    let mut values =
        vec![unsafe { MaybeUninit::<T>::zeroed().assume_init() }; capacity_bytes / element_size];
    let mut size = byte_size;
    status_to_result(operation, read(values.as_mut_ptr().cast(), &raw mut size))?;
    let written = size as usize;
    if written > capacity_bytes || written % element_size != 0 {
        return Err(AudioToolboxError::message(
            operation,
            format!("property returned {written} bytes for a {capacity_bytes}-byte buffer of {element_size}-byte elements"),
        ));
    }
    values.truncate(written / element_size);
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::{read_property, read_property_array};
    use crate::{AudioStreamBasicDescription, AudioValueRange, NO_ERR};

    #[test]
    fn read_property_rejects_a_short_payload() {
        let result = read_property::<AudioStreamBasicDescription>("test", |_, size| {
            unsafe { *size = 4 };
            NO_ERR
        });
        assert!(result.is_err());
    }

    #[test]
    fn read_property_accepts_an_exact_payload() {
        let value = read_property::<u32>("test", |data, size| {
            unsafe {
                data.cast::<u32>().write(7);
                assert_eq!(*size, 4);
            }
            NO_ERR
        })
        .expect("exact payload");
        assert_eq!(value, 7);
    }

    #[test]
    fn read_property_reports_the_framework_status() {
        let result = read_property::<u32>("test", |_, _| -50);
        assert!(result.is_err());
    }

    #[test]
    fn read_property_array_truncates_to_the_reported_size() {
        let values = read_property_array::<u32>("test", 16, |data, size| {
            unsafe {
                data.cast::<u32>().write(1);
                data.cast::<u32>().add(1).write(2);
                *size = 8;
            }
            NO_ERR
        })
        .expect("two elements");
        assert_eq!(values, vec![1, 2]);
    }

    #[test]
    fn read_property_array_rejects_partial_elements_and_growth() {
        assert!(read_property_array::<u32>("test", 6, |_, _| NO_ERR).is_err());
        assert!(read_property_array::<u32>("test", 8, |_, size| {
            unsafe { *size = 6 };
            NO_ERR
        })
        .is_err());
        assert!(
            read_property_array::<AudioValueRange>("test", 16, |_, size| {
                unsafe { *size = 32 };
                NO_ERR
            })
            .is_err()
        );
    }

    #[test]
    fn read_property_array_skips_the_call_for_an_empty_payload() {
        let values = read_property_array::<u32>("test", 0, |_, _| panic!("must not be called"))
            .expect("empty payload");
        assert!(values.is_empty());
    }
}
