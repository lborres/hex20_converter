// use hex;
use std::error::Error;
use std::fs;

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    // cli::init();
    let file = "target/debug/small_input.txt";

    let contents = fs::read_to_string(file)?;
    println!("{contents}");

    Ok(())
}

// fn checksum_gen -> 2Hex: u8

// fn rearrange_hex ->
