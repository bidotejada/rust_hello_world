use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let mut f = File::open("foo.txt").unwrap();
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer).unwrap();

    println!("The bytes: {:?}", &buffer[..n]);
}
