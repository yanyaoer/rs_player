extern crate rodio;

use std::io::BufReader;

fn rodio() {
    let path = std::env::args().nth(1).expect("no audio given");

    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);

    let file = std::fs::File::open(path).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}


fn main() {
  rodio()
}
