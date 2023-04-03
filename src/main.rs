use std::env;
use std::fs;
use std::process;

use sha256::Config;
use sha256::Sha256;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let str_bytes = "hello world".as_bytes();

    let mut str_binary = "".to_string();

    for char in str_bytes {
        str_binary += &format!("0{:b} ", char);
    }

    let mut sha256 = Sha256::default();

    Sha256::process_chunks(&mut sha256, str_bytes.to_vec());
    println!("{:#?}", str_binary);

    let binary = (fs::read("./testfile.txt")).expect("error reading file into binary");
    println!("{:#?}", binary);
    println!("{:#?}", binary.len());
}
