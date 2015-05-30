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
                self.join_game(gid, pid);
            }
            Event::PlaceBid(gid, pid, iid, qty, px) => {
                self.place_bid(gid, pid, iid, qty, px);
            }
        }
    }

    pub fn place_bid(&mut self,
                     gid: GameId, pid: PlayerId,
                     iid: ItemId, qty: Quantity,
                     px: Money) {
        assert_eq!(self.id, gid);
        match self.players.get_mut(&pid) {
            None => elog!("player {:?} not in game {:?}", pid, gid),
            Some(player) => player.place_bid(iid, qty, px),
        }
    }

    pub fn join_game(&mut self, gid: GameId, pid: PlayerId) {
        assert_eq!(self.id, gid);
        let player = Player::new(pid.clone(), self.data_pack.starting_money);
        self.players.insert(pid, player);
    }

    // TODO call from a task scheduler of sorts running on a separate thread
    pub fn game_loop(&self) {
        self.opening_auction();
        for i in 1..5 {
            self.common_auction();
        }
        self.closing_auction();
        self.resolve_winners();
    }

    pub fn opening_auction(&self) {
        // wait for bids
        self.resolve_bids();
    }

    pub fn common_auction(&self) {
        // wait for bids
        self.resolve_bids();
    }

    pub fn closing_auction(&self) {
        // wait for bids
        self.resolve_bids();
    }

    pub fn resolve_winners(&self) {
    }

    pub fn resolve_bids(&self) {
        // TODO higher bid wins and loses money

    }

    pub fn list_bids(&self) {
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use data_pack::*;
    use common::*;

    fn test_setup() -> (Game, GameId, ItemId, PlayerId, Quantity, Money) {
      let dp = DataPack::load(&mut File::open("res/demo_auction.json").unwrap());

      let gid = GameId("game1".to_string());
      let itemId = ItemId("item1".to_string());
      let pid = PlayerId("player1".to_string());
      let quant = Quantity(10);
      let sum = Money(10);
      let mut g  = Game::new(gid.clone(), dp);
      (g, gid, itemId, pid, quant, sum)
    }

    #[test]
    fn test_bid() {
      let (mut g,  gid, itemId, pid, quant, sum) = test_setup();

      let join_ev = Event::JoinGame(gid.clone(), pid.clone());
      let bid_ev  = Event::PlaceBid(gid.clone(), pid.clone(), itemId, quant, sum);
      g.apply_event(join_ev);
      g.apply_event(bid_ev);

      g.resolve_bids();
      g.list_bids();
      // TODO check bids resolved correctly
    }

    #[test]
    fn test_game_loop() {
      let (mut g,  gid, itemId, pid, quant, sum) = test_setup();
      g.game_loop();
    }
}
