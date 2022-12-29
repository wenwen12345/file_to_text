use base64::{decode, encode};
use std::fs;
use std::io::{Read, Write};
use clap::{Parser, Subcommand};

// an app to make file to text
#[derive(Parser, Debug)]
#[command(author = "wen", version = "V1.1.0", about, long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub mode: Config
}

#[derive(Subcommand, Debug)]
pub enum Config {
    // encode a file
    Encode{
        filename: String
    },
    // decode a file
    Decode {
        filename:String,
        text: String
    },
}

impl App {
    // need to parse before call
    pub fn encode(&self) -> String {
        match &self.mode {
            Config::Encode{filename} => {
                let tmp = get_file_as_byte_vec(&filename);
                encode(&tmp)
            }
            Config::Decode{..} => panic!("The config is not decode"),
        }
    }
    // need to parse before call
    pub fn decode(&self) {
        match &self.mode {
            Config::Encode{..} => panic!("The config is not encode"),
            Config::Decode{filename, text} => {
                write_file_as_byte_vec(&filename, &decode(&text).expect("the text is wrong"));
            }
        }
    }
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = fs::File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn write_file_as_byte_vec(filename: &String, arr: &Vec<u8>) {
    let mut f = fs::File::create(&filename).expect("file not found");
    f.write(&arr).expect("Can not write to file ");
}
