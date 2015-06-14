use std::io;
use std::fmt;
use std::collections::HashMap;

use rustc_serialize;
use rustc_serialize::json;

use common::*;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Agenda {
    pub id: ItemId,
    pub title: String,
    pub flavour_text: String,
    pub cost: Resources,
    pub value: Money,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Asset {
    pub id: ItemId,
    pub title: String,
    pub flavour_text: String,
    pub provides: Resources,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub enum Item {
    Agenda(Agenda),
    Asset(Asset),
}

impl Item {
    pub fn id(&self) -> ItemId {
        match self {
            &Item::Agenda(ref a) => a.id.clone(),
            &Item::Asset(ref a) => a.id.clone(),
        }
    }
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Auction {
    pub id: AuctionId,
    pub items: Vec<(Quantity, ItemId)>,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct DataPack {
    pub items: HashMap<ItemId, Item>,
    pub auctions: Vec<Auction>,
    pub starting_money: Money,
}

impl DataPack {
    pub fn load(rdr: &mut io::Read) -> DataPack {
        let json = json::Json::from_reader(rdr).unwrap();
        let mut decoder = json::Decoder::new(json);
        rustc_serialize::Decodable::decode(&mut decoder).unwrap()
    }

    pub fn auction(&self, aid: &AuctionId) -> Option<&Auction> {
        for auction in &self.auctions {
            if &auction.id == aid {
                return Some(auction);
            }
        }
        return None;
    }
}

impl fmt::Display for DataPack {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&format!("{:?}", self))
    }
}
