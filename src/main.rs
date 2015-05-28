extern crate venice;

use venice::*;
use std::fs::File;

fn main() {
    let dp = DataPack::load(&mut File::open("res/demo_auction.json").unwrap());
    println!("data_pack: {}", dp);
}
