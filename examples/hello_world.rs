use rodio::{Decoder, speakers::SpeakersBuilder, source::Source, buffer::SamplesBuffer};
use static_cell::StaticCell;
use std::{io::Cursor, num::NonZero};
use libsam_rs::speak_words;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vec = speak_words("Hello, world!").unwrap();
    let samples = rodio::conversions::SampleTypeConverter::new(vec.into_iter()).collect::<Vec<f32>>();
    let source = SamplesBuffer::new(NonZero::new(1).unwrap(), NonZero::new(22050).unwrap(), samples);
    //let source = Decoder::new(cursor).unwrap();
    // Get an OS-Sink handle to the default physical sound device.
    // Note that the playback stops when the handle is dropped.//!
    //let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    let speakers = SpeakersBuilder::new()
    .default_device()?
    .default_config()?
    .open_mixer()?;
    let mixer = speakers.mixer();
    mixer.add(source);

    std::thread::sleep(std::time::Duration::from_secs(10));
    Ok(())
}
