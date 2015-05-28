extern crate rustc_serialize;

mod data_pack;

#[derive(Debug)]
pub struct Resources {
    force: u32,
    popularity: u32,
    influence: u32,
}

#[derive(Debug)]
pub struct Money(u32);

#[derive(Debug)]
pub struct Agenda {
    id: String,
    title: String,
    flavour_text: String,
    cost: Resources,
    value: Money,
}

#[derive(Debug)]
pub struct Asset {
    id: String,
    title: String,
    flavour_text: String,
    provides: Resources,
}

#[derive(Debug)]
pub enum Item {
    Agenda(Agenda),
    Asset(Asset),
}

#[derive(Debug)]
pub struct Auction {
    items: Vec<Item>,
}

impl Auction {
    pub fn new() -> Auction {
        Auction { items: vec![], }
    }
}

pub struct AuctionSchedule {
    auctions: Vec<Auction>,
}
