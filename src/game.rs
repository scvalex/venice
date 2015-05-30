use std::collections::{LinkedList, HashMap, HashSet};
use std::iter::FromIterator;
use std::io::Write;

use common::*;
use data_pack::*;

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

pub enum Event<'a> {
    JoinGame(GameId, &'a PlayerId),
    PlaceBid(GameId, &'a PlayerId, &'a ItemId, Quantity, Money),
}

#[derive(Debug)]
pub struct Game<'a> {
    id: GameId,
    data_pack: DataPack,
    completed_auctions: Vec<CompletedAuction<'a>>,
    pending_auctions: LinkedList<AuctionId>,
    players: HashMap<&'a PlayerId, Player<'a>>,
}

impl<'a> Game<'a> {
    pub fn new(id: GameId, data_pack: DataPack) -> Game<'a> {
        let pending_auctions =
            FromIterator::from_iter(data_pack.auctions.iter().map(|a| a.id.clone()));
        Game {
            id: id,
            data_pack: data_pack,
            completed_auctions: vec![],
            pending_auctions: pending_auctions,
            players: HashMap::new(),
        }
    }

    pub fn apply_event(&mut self, ev: &'a Event) {
        match ev {
            &Event::JoinGame(ref gid, pid) => {
                assert_eq!(&self.id, gid);
                let player = Player::new(pid, self.data_pack.starting_money);
                self.players.insert(pid, player);
            }
            &Event::PlaceBid(ref gid, pid, iid, qty, px) => {
                assert_eq!(&self.id, gid);
                match self.players.get_mut(pid) {
                    None => elog!("player {:?} not in game {:?}", pid, gid),
                    Some(player) => player.place_bid(iid, qty, px),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use data_pack::*;
    use common::*;

    #[test]
    fn game_works() {
      let _dp = DataPack::load(&mut File::open("res/test_data_pack.json").unwrap());
      let _g  = Game::new(GameId("1".to_string()), _dp);
    }
}
