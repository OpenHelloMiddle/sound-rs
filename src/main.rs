use rodio::{Decoder, DeviceSinkBuilder, Player};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut handle = DeviceSinkBuilder::open_default_sink()
        .expect("Failed to open default audio sink");
    handle.log_on_drop(false);
    let player = Player::connect_new(handle.mixer());
    for path_str in &args[1..] {
        let path = Path::new(path_str);
        let file = File::open(path).expect("Failed to open audio file");
        let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio file");
        player.append(source);
    }
    player.sleep_until_end();
}