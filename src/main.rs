use file_to_text::Config;
use std::env;
fn main() {
    let conf = Config::from_cli(env::args());
    match conf {
        Config::Encode(_) => {
            print!("{}", conf.encode());
        },
        Config::Decode(_, _) => {
            conf.decode();
            print!("decode success")
        }
    }
}
