// extern crate ffmpeg_next as ffmpeg;

pub use ffmpeg_next as ffmpeg;

pub mod video;

pub fn init() {
    ffmpeg::init().unwrap();
}
