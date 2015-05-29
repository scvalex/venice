use std::collections::{LinkedList, HashMap, HashSet};
use std::iter::FromIterator;

use data_pack::*;

#[derive(Debug)]
pub struct GameId(String);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PlayerId(String);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Quantity(u32);

#[derive(Debug)]
pub struct Player {
    id: PlayerId,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Bid<'a> {
    player: &'a PlayerId,
    quantity: Quantity,
    price: Money,
}

#[derive(Debug)]
pub struct Bids<'a> {
    winning_bids: HashSet<Bid<'a>>,
    other_bids: HashSet<Bid<'a>>,
}

#[derive(Debug)]
pub struct CompletedAuction<'a> {
    id: AuctionId,
    bids: HashMap<&'a ItemId, Bids<'a>>,
}

#[derive(Debug)]
pub struct Game<'a> {
    id: &'a GameId,
    data_pack: &'a DataPack,
    completed_auctions: Vec<CompletedAuction<'a>>,
    pending_auctions: LinkedList<&'a AuctionId>,
    players: HashMap<&'a PlayerId, Player>,
}

impl<'a> Game<'a> {
    pub fn new(id: &'a GameId, data_pack: &'a DataPack) -> Game<'a> {
        let pending_auctions =
            FromIterator::from_iter(data_pack.auctions.iter().map(|a| &a.id));
        Game {
            id: id,
            data_pack: &data_pack,
            completed_auctions: vec![],
            pending_auctions: pending_auctions,
            players: HashMap::new(),
        }
    }
}
