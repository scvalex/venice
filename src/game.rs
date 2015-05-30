use std::collections::{LinkedList, HashMap, HashSet};
use std::iter::FromIterator;
use std::io::Write;

use common::*;
use data_pack::*;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Bid {
    player:  PlayerId,
    item:  ItemId,
    quantity: Quantity,
    price: Money,
}

#[derive(Debug)]
struct Player {
    id:  PlayerId,
    resources: Resources,
    money: Money,
    bids: HashSet<Bid>,
}

impl Player {
    fn new(id: PlayerId, money: Money) -> Player {
        Player {
            id: id,
            resources: Resources { force: 0, influence: 0, popularity: 0, },
            money: money,
            bids: HashSet::new(),
        }
    }

    fn place_bid(&mut self, iid:  ItemId, qty: Quantity, px: Money) {
        self.bids.insert(Bid { player: self.id.clone(), item: iid, quantity: qty, price: px});
    }
}

#[derive(Debug)]
struct Bids {
    winning_bids: HashSet<Bid>,
    other_bids: HashSet<Bid>,
}

#[derive(Debug)]
struct CompletedAuction {
    id: AuctionId,
    bids: HashMap< ItemId, Bids>,
}

pub enum Event {
    JoinGame(GameId,  PlayerId),
    PlaceBid(GameId,  PlayerId,  ItemId, Quantity, Money),
}

#[derive(Debug)]
pub struct Game {
    id: GameId,
    data_pack: DataPack,
    completed_auctions: Vec<CompletedAuction>,
    pending_auctions: LinkedList<AuctionId>,
    players: HashMap< PlayerId, Player>,
}

impl Game {
    pub fn new(id: GameId, data_pack: DataPack) -> Game {
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

    pub fn apply_event(&mut self, ev: Event) {
        match ev {
            Event::JoinGame(gid, pid) => {
                assert_eq!(self.id, gid);
                let player = Player::new(pid.clone(), self.data_pack.starting_money);
                self.players.insert(pid, player);
            }
            Event::PlaceBid(gid, pid, iid, qty, px) => {
                assert_eq!(self.id, gid);
                match self.players.get_mut(&pid) {
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
      let _dp = DataPack::load(&mut File::open("res/demo_auction.json").unwrap());
      let gid = GameId("1".to_string());
      let mut _g  = Game::new(GameId("1".to_string()), _dp);
      let pid = PlayerId("player1".to_string());
      let e = Event::JoinGame(gid, pid);
      _g.apply_event(e);
    }
}
