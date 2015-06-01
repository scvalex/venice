use std::collections::{LinkedList, HashMap, HashSet};
use std::iter::FromIterator;
use std::io::Write;
use std::thread;
use std::sync::Mutex;

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
    agendas: Vec<Agenda>,
    assets: Vec<Asset>
}

impl Player {
    fn new(id: PlayerId, money: Money) -> Player {
        Player {
            id: id,
            resources: Resources { force: 0, influence: 0, popularity: 0, },
            money: money,
            bids: HashSet::new(),
            agendas: Vec::new(),
            assets: Vec::new()
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
    min_players: usize,
    assets: Vec<Asset>,
    agendas: Vec<Agenda>,
}

impl Game {
    pub fn new(id: GameId, data_pack: DataPack) -> Game {
        let pending_auctions =
            FromIterator::from_iter(data_pack.auctions.iter().map(|a| a.id.clone()));
        Game {
            id: id,
            data_pack: data_pack.clone(),
            completed_auctions: vec![],
            pending_auctions: pending_auctions,
            players: HashMap::new(),
            min_players: 2,
            assets: data_pack.assets,
            agendas: data_pack.agendas
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

    fn place_bid(&mut self,
                     gid: GameId, pid: PlayerId,
                     iid: ItemId, qty: Quantity,
                     px: Money) {
        assert_eq!(self.id, gid);
        match self.players.get_mut(&pid) {
            None => elog!("player {:?} not in game {:?}", pid, gid),
            Some(player) => player.place_bid(iid, qty, px),
        }
    }

    fn join_game(&mut self, gid: GameId, pid: PlayerId) {
        assert_eq!(self.id, gid);
        let player = Player::new(pid.clone(), self.data_pack.starting_money);
        self.players.insert(pid, player);
    }

    // main game loop. run on a separate thread  since it will sleep waiting for events
    pub fn game_loop(&mut self) {
        self.wait_for_players();
        self.opening_auction();
        for i in 1..5 {
            self.common_auction();
        }
        self.closing_auction();
        self.resolve_winners();
    }

    fn wait_for_players(&self) {
        let mut done = false;
        while !done {
            thread::sleep_ms(1000);
            if self.players.len() >= self.min_players {
                done = true;
            }
        }
    }

    fn opening_auction(&self) {
        // wait for bids
        self.resolve_bids();
    }

    fn common_auction(&self) {
        // wait for bids
        self.resolve_bids();
    }

    fn closing_auction(&self) {
        // wait for bids
        self.resolve_bids();
    }

    fn resolve_winners(&self) {
    }

    fn resolve_bids(&self) {
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
    use std::thread;
    use std::sync::Mutex;

    fn test_setup() -> (Game, GameId, ItemId, PlayerId, Quantity, Money, DataPack) {
      let dp = DataPack::load(&mut File::open("res/demo_auction.json").unwrap());

      let gid = GameId("game1".to_string());
      let itemId = ItemId("item1".to_string());
      let pid = PlayerId("player1".to_string());
      let quant = Quantity(10);
      let sum = Money(10);
      let mut g  = Game::new(gid.clone(), dp.clone());
      (g, gid, itemId, pid, quant, sum, dp)
    }

    #[test]
    fn test_bid() {
      let (mut g,  gid, itemId, pid, quant, sum, _) = test_setup();
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
      let (mut g,  gid, itemId, pid, quant, sum, dp) = test_setup();
      let gsafe = Arc::new(Mutex::new(g));

      let grec = gsafe.clone();
      let handle = thread::spawn(move || {
          let mut g = grec.lock().unwrap();
          g.game_loop();
      });

      for i in 0..2 {
          let pid = PlayerId(i.to_string());
          let join_ev = Event::JoinGame(gid.clone(), pid);
          gsafe.lock().unwrap().apply_event(join_ev);
      }

      // TODO check some other stuff

      // wait for the game loop to finish
      handle.join();
    }
}
