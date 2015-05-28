use rustc_serialize;
use rustc_serialize::json;
use std::io::{Read, Error};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Resources {
    force: u32,
    popularity: u32,
    influence: u32,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Money(u32);

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct ItemId(String);

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

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Auction {
    items: Vec<ItemId>,
}

impl Auction {
    pub fn new() -> Auction {
        Auction { items: vec![], }
    }
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct DataPack {
    items: Vec<ItemId>,
    auctions: Vec<Auction>,
}

impl DataPack {
    pub fn load(rdr: &mut Read) -> DataPack {
        let json = json::Json::from_reader(rdr).unwrap();
        let mut decoder = json::Decoder::new(json);
        rustc_serialize::Decodable::decode(&mut decoder).unwrap()
    }
}

impl Display for DataPack {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&format!("{:?}", self))
    }
}
