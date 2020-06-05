use std::fs::File;
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use minimp3::{Decoder, Frame, Error};


pub fn cpal() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    let format_range = device
        .supported_output_formats()
        .unwrap()
        .next()
        .expect("Failed to get endpoint format");

    let mut format = format_range.with_max_sample_rate();
    format.sample_rate = cpal::SampleRate(44_100);
    let event_loop = host.event_loop();
    let stream_id = event_loop
        .build_output_stream(&device, &format)
        .expect("Failed to create a voice");

    let mut decoder = Decoder::new(File::open("/tmp/01.mp3").unwrap());

    loop {
        match decoder.next_frame() {
            Ok(Frame { data, sample_rate, channels, .. }) => {
                println!("Decoded {} samples", data.len() / channels)
            },
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }

    event_loop.play_stream(stream_id).expect("failed to play_stream");
}
