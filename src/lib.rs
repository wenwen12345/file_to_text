use base64::{decode, encode};
use std::env::Args;
use std::fs::{self, File};
use std::io::{Read, Write};

pub enum Config {
    Encode(String),
    Decode(String, String),
}

impl Config {
    pub fn from_cli(mut arg: Args) -> Config {
        arg.next();
        let mode = arg.next().expect(
            "Err input \nUsage:\n\tfile_to_text encode filepath\nfile_to_text decode filepath text",
        );
        match mode.as_str() {
            "encode" => {
                let filename = arg.next().expect("Err input \nUsage:\n\tfile_to_text encode filepath\nfile_to_text decode filepath text");
                Config::Encode(filename)
            }
            "decode" => {
                let (filename, text) = (arg.next().expect("Err input \nUsage:\n\tfile_to_text encode filepath\nfile_to_text decode filepath text"), arg.next().expect("Err input \nUsage:\n\tfile_to_text encode filepath\nfile_to_text decode filepath text"));
                Config::Decode(filename, text)
            }
            _ => {
                print!("{}", mode);
                panic!("\nErr input \nUsage:\n\tfile_to_text encode filepath\nfile_to_text decode filepath text\n")
            }
        }
    }

    pub fn encode(&self) -> String {
        match self {
            Config::Encode(filename) => {
                let tmp = get_file_as_byte_vec(&filename);
                encode(&tmp)
            }
            Config::Decode(_, _) => panic!("The config is not decode"),
        }
    }
    pub fn decode(&self) {
        match self {
            Config::Encode(_) => panic!("The config is not encode"),
            Config::Decode(filename, text) => {
                write_file_as_byte_vec(&filename, &decode(&text).expect("the text is wrong"));
            }
        }
    }
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn write_file_as_byte_vec(filename: &String, arr: &Vec<u8>) {
    let mut f = File::create(&filename).expect("file not found");
    f.write(&arr).expect("Can not write to file ");
}
