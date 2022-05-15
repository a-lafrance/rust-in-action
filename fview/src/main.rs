use std::{
    fs::File,
    io::{BufReader, Read},
};
use clap::Parser;

const BYTES_PER_LINE: usize = 16;

#[derive(Parser)]
struct Args {
    #[clap(help = "File to examine")]
    input: String,
}

fn main() {
    let input = Args::parse().input;
    let file = File::open(&input).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = [0; BYTES_PER_LINE];
    let mut line = 0;

    while reader.read_exact(&mut buf).is_ok() {
        print!("[0x{:08x}] ", BYTES_PER_LINE * line);

        for byte in buf {
            print!("{:02x} ", byte);
        }

        println!();
        line += 1;
    }
}
