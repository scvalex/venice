use std::collections::{LinkedList, HashMap, HashSet};
use std::iter::FromIterator;

use common::*;
use data_pack::*;
use event::Event;

#[derive(Debug)]
pub struct Player<'a> {
    id: &'a PlayerId,
}

impl<'a> Player<'a> {
    fn new(id: &PlayerId) -> Player {
        Player { id: id, }
    }
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
    players: HashMap<&'a PlayerId, Player<'a>>,
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

    pub fn apply_event(&mut self, ev: &'a Event) {
        match ev {
            &Event::JoinGame(gid, pid) => {
                assert_eq!(self.id, gid);
                self.players.insert(pid, Player::new(pid));
            }
        }
    }
}
