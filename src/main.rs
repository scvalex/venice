struct Resources {
    force: u32,
    popularity: u32,
    influence: u32,
}

struct Money(u32);

struct Agenda {
    id: String,
    title: String,
    flavour_text: String,
    cost: Resources,
    value: Money,
}

struct Asset {
    id: String,
    title: String,
    flavour_text: String,
    provides: Resources,
}

enum Item {
    Agenda(Agenda),
    Asset(Asset),
}

fn main() {
    println!("Hello, world!");
}
