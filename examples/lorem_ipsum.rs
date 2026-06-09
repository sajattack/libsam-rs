use rodio::{speakers::SpeakersBuilder, buffer::SamplesBuffer};
use std::{num::NonZero};
use libsam_rs::speak_words;
use std::boxed::Box;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lorem_str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium. Totam rem aperiam, eaque ipsa quae ab illo inventore ver.";
    let vec = speak_words(lorem_str).unwrap();
    let samples = rodio::conversions::SampleTypeConverter::new(vec.into_iter()).collect::<Vec<f32>>();
    let source = SamplesBuffer::new(NonZero::new(1).unwrap(), NonZero::new(22050).unwrap(), samples);
    
    let speakers = SpeakersBuilder::new()
    .default_device()?
    .default_config()?
    .open_mixer()?;
    let mixer = speakers.mixer();
    mixer.add(source);

    std::thread::sleep(std::time::Duration::from_secs(100));
    Ok(())
}
