use common::*;

pub enum Event<'a> {
    JoinGame(&'a GameId, &'a PlayerId),
    PlaceBid(&'a GameId, &'a PlayerId, &'a ItemId, Quantity, Money),
}
