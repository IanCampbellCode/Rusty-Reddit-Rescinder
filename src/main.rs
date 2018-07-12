extern crate toml;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Reading key from Secret_Key.toml....");
    let name = "Secret_Key.toml";
    let mut secret_key = String::new();

    //Read secret key file and parse value
    match File::open(name).and_then(|mut f| {
        f.read_to_string(&mut secret_key)}) {
        Ok(_) => (),
        Err (_) => {
            println!("Failed to read file: {}", name);
        },
    }
    let secret_key: toml::Value = toml::from_str(&secret_key).unwrap();

    
}
