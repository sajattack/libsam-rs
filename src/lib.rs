//! libSAM-rs
//! sys/safe bindings for Software Automatic Mouth
//! this runs using my fork of SAM
//! which provides utility functions for setting Tts values
//! and a utility function for rendering messages (they have to be formatted)
//! this does build SAM for your target arch also and gets statically linked
//! 
//! If you have not heard of SAM it was used by the game FAITH: The Unholy Trinity
//! due to having 4 values it can be really easy to use if you just randomize the values by 32-64

#![no_std]

extern crate alloc;

use alloc::string::ToString;
use alloc::vec;
use core::option::Option;

use core::ffi::c_char;

pub mod sys;

/// A enum containg all errors that TTS can return
#[derive(Debug, Clone, Copy)]
pub enum TTSError {
    /// the string *you* passed contains a null, dont do that
    ContainsNull,
    /// error id from the libSAM, will mabey split this into values l8r
    Code(i32),
    BufferTooSmall,
    NullErr,
}



/// set SAM tts values (0/None sets value to default)
pub fn set_speech_values(
    pitch: Option<u8>,
    speed: Option<u8>,
    throat: Option<u8>,
    mouth: Option<u8>,
) {
    unsafe {
        sys::setupSpeak(
            pitch.unwrap_or(0),
            speed.unwrap_or(0),
            throat.unwrap_or(0),
            mouth.unwrap_or(0),
        )
    }
}

/// internal function to render a string into PCM audio
/// SAFTEY: chunk must be at most 255 bytes long
unsafe fn render_chunk(chunk: &str, buf: &mut [u8]) -> Result<(),TTSError> {
    //let mut bytes: Vec<i8> = chunk.bytes().map(|b|{std::mem::transmute(b)}).collect();
    //bytes.push(0);
    let mut bytes = chunk.to_string().into_bytes();
    bytes.push(0);
    let cstr = core::ffi::CStr::from_bytes_with_nul(&bytes).map_err(|_e|TTSError::NullErr)?;
    let ptr = sys::speakText(cstr.as_ptr() as *mut c_char);
    let res = ptr.read();
    if res.res != 1 {
        drop(bytes);
        return Err(TTSError::Code(res.res))
    }
    if buf.len() < res.buf_size as usize {
        return Err(TTSError::BufferTooSmall)
    }
    let resbuf = core::slice::from_raw_parts(res.buf, res.buf_size as usize);
    let resbuf = core::mem::transmute::<&[c_char], &[u8]>(resbuf);
    buf[..res.buf_size as usize].copy_from_slice(resbuf); 
    Ok(())
}

/// Speaks the chosen text as a message
pub fn speak_words(tospeak: &str, buf: &mut [u8]) -> Result<(), TTSError> {
    if tospeak.len()<=255 {
        unsafe {render_chunk(tospeak, buf)?}
    } else {
        let words = tospeak.split(' ');
        let mut small = vec![];
        for word in words {
            if small.iter().map(|x:&&str| {x.len() }).fold(0,|acc, x| acc + x)+word.len() < 254 {
                small.push(word);
            } else {
                small.push("\0");
                unsafe {render_chunk(small.join(" ").as_str(), buf)?}
            }
        }
        unsafe {render_chunk(small.join(" ").as_str(), buf)?};
    };
    Ok(())
}
