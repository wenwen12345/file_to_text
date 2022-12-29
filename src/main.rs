use clap::Parser;
use file_to_text::{App, Config};

fn main() {
    let conf = App::parse();
    match conf.mode {
        Config::Encode{..} => {
            print!("{}", conf.encode());
        },
        Config::Decode{..} => {
            conf.decode();
            print!("decode success")
        }
    }
}
