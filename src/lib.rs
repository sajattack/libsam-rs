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
use core::ffi::c_char;
//use core::ffi::c_void;
use alloc::ffi::NulError;
use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;


pub mod sys;

/// unsigned 8-bit mono pcm @ 22050 hz audio returned by SAM
pub type SAMAudio = Vec<u8>;

/// A enum containg all errors that TTS can return
#[derive(Debug,Clone,Copy)]
pub enum TTSError {
    /// the string *you* passed contains a null, dont do that
    ContainsNull,
    /// error id from the libSAM, will mabey split this into values l8r
    Code(i32),
    Other,
}

/// quick impl so i dont have to catch it and it can just be questioned
impl From<NulError> for TTSError {
    fn from(_value: NulError) -> Self {
        TTSError::ContainsNull
    }
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
/// SAFTEY: chunk must be at most 100 bytes long
unsafe fn render_chunk(chunk: &str) -> Result<Vec<u8>,TTSError> {
    let mut bytes: Vec<c_char> = chunk.bytes().map(|b|{b as c_char}).collect();
    bytes.push(0);
    let ptr = sys::speakText(bytes.as_mut_ptr());
    let res = ptr.read();
    if res.res != 1 {
        //libc::free(ptr as *mut c_void);
        return Err(TTSError::Code(res.res))
    }
    let buf = core::slice::from_raw_parts(res.buf, res.buf_size as usize);
    let result = buf.into_iter().map(|b| *b as u8).collect();
    Ok(result)
}

/// Speaks the chosen text as a message
pub fn speak_words(tospeak: &str) -> Result<SAMAudio, TTSError> {
    let bytes: Vec<u8> = if tospeak.len()<=100{
        unsafe {render_chunk(tospeak)?}
    } else {
            let mut words = tospeak.split(' ').peekable();
            let mut result: Vec<u8> = vec![];
            let mut processed_char_len = 0;
            'outer: loop 
            {
                let mut small = String::new();
                'inner: loop 
                {

                    if small.len()+words.peek().ok_or(TTSError::Other)?.len() > 100 {
                        break 'inner;
                    } else {
                        small.push_str(words.next().ok_or(TTSError::Other)?);
                        small.push_str(" ");
                    }
                }
                result.append(&mut unsafe {render_chunk(small.as_str())?});
                processed_char_len += small.len();
                if tospeak.len() - processed_char_len <= 100
                {
                    break 'outer;
                }
            } 
            result.append(&mut unsafe { render_chunk(&tospeak[processed_char_len..])?});
        result
    };
    Ok(bytes)
}
