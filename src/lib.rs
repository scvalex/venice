#[derive(Debug)]
struct Resources {
    force: u32,
    popularity: u32,
    influence: u32,
}

#[derive(Debug)]
struct Money(u32);

#[derive(Debug)]
struct Agenda {
    id: String,
    title: String,
    flavour_text: String,
    cost: Resources,
    value: Money,
}

#[derive(Debug)]
struct Asset {
    id: String,
    title: String,
    flavour_text: String,
    provides: Resources,
}

#[derive(Debug)]
enum Item {
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
