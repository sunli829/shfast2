#[macro_use]
extern crate anyhow;

use crate::parser::MktdtParser;
use std::time::Instant;

mod decimal;
mod ftcodec;
mod parser;

fn main() {
    let filenames = ["data/mktdt00.txt", "data/mktdt00.txt"];

    let mut parser = MktdtParser::new();
    for _ in 0..10 {
        for filename in &filenames {
            let s = Instant::now();
            parser.parse_file(filename).unwrap();
            println!("parse used: {:?} ms", s.elapsed().as_millis());
        }
    }
}
