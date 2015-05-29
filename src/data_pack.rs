use std::io;
use std::fmt;

use rustc_serialize;
use rustc_serialize::json;

use common::*;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Agenda {
    id: ItemId,
    title: String,
    flavour_text: String,
    cost: Resources,
    value: Money,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Asset {
    id: ItemId,
    title: String,
    flavour_text: String,
    provides: Resources,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum Item {
    Agenda(Agenda),
    Asset(Asset),
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Auction {
    pub id: AuctionId,
    items: Vec<(u32, ItemId)>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct DataPack {
    pub items: Vec<ItemId>,
    pub auctions: Vec<Auction>,
    pub starting_money: Money,
}

impl DataPack {
    pub fn load(rdr: &mut io::Read) -> DataPack {
        let json = json::Json::from_reader(rdr).unwrap();
        let mut decoder = json::Decoder::new(json);
        rustc_serialize::Decodable::decode(&mut decoder).unwrap()
    }
}

impl fmt::Display for DataPack {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&format!("{:?}", self))
    }
}
