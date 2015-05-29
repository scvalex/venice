use std::collections::{LinkedList, HashMap, HashSet};
use std::iter::FromIterator;
use std::io::Write;

use common::*;
use data_pack::*;
use event::Event;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Bid<'a> {
    player: &'a PlayerId,
    item: &'a ItemId,
    quantity: Quantity,
    price: Money,
}

#[derive(Debug)]
struct Player<'a> {
    id: &'a PlayerId,
    resources: Resources,
    money: Money,
    bids: HashSet<Bid<'a>>,
}

impl<'a> Player<'a> {
    fn new(id: &PlayerId, money: Money) -> Player {
        Player {
            id: id,
            resources: Resources { force: 0, influence: 0, popularity: 0, },
            money: money,
            bids: HashSet::new(),
        }
    }

    fn place_bid(&mut self, iid: &'a ItemId, qty: Quantity, px: Money) {
        self.bids.insert(Bid { player: self.id, item: iid, quantity: qty, price: px});
    }
}

#[derive(Debug)]
struct Bids<'a> {
    winning_bids: HashSet<Bid<'a>>,
    other_bids: HashSet<Bid<'a>>,
}

#[derive(Debug)]
struct CompletedAuction<'a> {
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
                let player = Player::new(pid, self.data_pack.starting_money);
                self.players.insert(pid, player);
            }
            &Event::PlaceBid(gid, pid, iid, qty, px) => {
                assert_eq!(self.id, gid);
                match self.players.get_mut(pid) {
                    None => elog!("player {:?} not in game {:?}", pid, gid),
                    Some(player) => player.place_bid(iid, qty, px),
                }
            }
        }
    }
}
