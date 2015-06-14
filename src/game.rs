use std::collections::{LinkedList, HashMap, HashSet};
use std::iter::FromIterator;
use std::io::Write;
use std::cmp;

/// No threads. No mutexes. Game should just be a container
/// for data and the logic which manipulates that data.
/// use std::thread;

use common::*;
use data_pack::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bid {
    player: PlayerId,
    item: ItemId,
    quantity: u32,
    price: u32,
}

#[derive(Debug)]
struct Player {
    id: PlayerId,
    money: u32,
    bids: HashSet<Bid>,
    agendas: Vec<(u32, Agenda)>,
    assets: Vec<(u32, Asset)>
}

impl Player {
    fn new(id: PlayerId, money: u32) -> Player {
        Player {
            id: id,
            money: money,
            bids: HashSet::new(),
            agendas: Vec::new(),
            assets: Vec::new()
        }
    }

    fn place_bid(&mut self, iid: ItemId, qty: u32, px: u32) {
        self.bids.insert(Bid { player: self.id.clone(), item: iid, quantity: qty, price: px});
    }

    fn clear_bids(&mut self) {
        self.bids.clear();
    }

    fn resources(&self) -> Resources {
        let mut r = Resources {force: 0, popularity: 0, influence: 0};
        for &(ref qty, ref a) in &self.assets {
            r.force += qty * a.provides.force;
            r.popularity += qty * a.provides.popularity;
            r.influence += qty * a.provides.influence;
        }
        r
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
    bids: HashMap<ItemId, Bids>,
}

pub enum Event {
    JoinGame(GameId, PlayerId),
    PlaceBid(GameId, PlayerId, ItemId, Quantity, Money),
    RunAuction(GameId),
}

#[derive(Debug)]
pub struct Game {
    id: GameId,
    data_pack: DataPack,
    completed_auctions: Vec<CompletedAuction>,
    pending_auctions: LinkedList<AuctionId>,
    players: HashMap<PlayerId, Player>,
    min_players: usize,
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
        }
    }

    pub fn apply_event(&mut self, ev: Event) {
        match ev {
            Event::JoinGame(gid, pid) => {
                self.join_game(gid, pid);
            }
            Event::PlaceBid(gid, pid, iid, Quantity(qty), Money(px)) => {
                self.place_bid(gid, pid, iid, qty, px);
            }
            Event::RunAuction(gid) => {
                self.run_auction(gid);
            }
        }
    }

    fn place_bid(&mut self,
                 gid: GameId, pid: PlayerId,
                 iid: ItemId, qty: u32,
                 px: u32) {
        assert_eq!(self.id, gid);
        match self.players.get_mut(&pid) {
            None => elog!("player {:?} not in game {:?}", pid, gid),
            Some(player) => player.place_bid(iid, qty, px),
        }
    }

    fn join_game(&mut self, gid: GameId, pid: PlayerId) {
        assert_eq!(self.id, gid);
        let Money(starting_money) = self.data_pack.starting_money;
        let player = Player::new(pid.clone(), starting_money);
        self.players.insert(pid, player);
    }

    fn run_auction(&mut self, gid: GameId) {
        assert_eq!(self.id, gid);
        match self.pending_auctions.pop_front() {
            None => {
                elog!("no pending auctions");
            }
            Some(aid) => {
                let auction = self.data_pack.auction(&aid).unwrap();
                for &(qty, ref iid) in &auction.items {
                    // This is all ridiculously hard to write.
                    let mut bids: Vec<Bid> = vec![];
                    for p in self.players.values() {
                        for b in &p.bids {
                            if &b.item == iid {
                                bids.push(b.to_owned());
                            }
                        }
                    }
                    bids.sort_by(|ref b1, ref b2| {
                        b2.price.cmp(&b1.price)
                    });
                    let Quantity(mut qty) = qty;
                    for b in &bids {
                        let p = self.players.get_mut(&b.player).unwrap();
                        let it = self.data_pack.items.get(&b.item).unwrap();
                        let won_qty = cmp::max(qty, cmp::min(b.quantity, p.money / b.price));
                        print!("{:?} won {} of {}\n", p.id, won_qty, it.id());
                        match it {
                            &Item::Agenda(ref a) => p.agendas.push((won_qty, a.to_owned())),
                            &Item::Asset(ref a) => p.assets.push((won_qty, a.to_owned())),
                        }
                        p.money -= won_qty * b.price;
                        qty -= won_qty;
                        if qty == 0 {
                            break;
                        }
                    }
                }
                for (_, p) in self.players.iter_mut() {
                    p.clear_bids();
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

    fn test_setup() -> (Game, GameId, ItemId, PlayerId, Quantity, Money, DataPack) {
        let dp = DataPack::load(&mut File::open("res/demo_auction.json").unwrap());
        let gid = GameId("game1".to_string());
        let itemId = "the-times".to_string();
        let pid = PlayerId("mario".to_string());
        let mut g = Game::new(gid.clone(), dp.clone());
        (g, gid, itemId, pid, Quantity(10), Money(10), dp)
    }

    #[test]
    fn test_auction1() {
        let (mut g, gid, itemId, pid, quant, px, _) = test_setup();
        g.apply_event(Event::JoinGame(gid.clone(), pid.clone()));
        g.apply_event(Event::PlaceBid(gid.clone(), pid.clone(), itemId, quant, px));
        assert_eq!(g.players.get(&pid).unwrap().money, 300);
        g.apply_event(Event::RunAuction(gid.clone()));
        assert_eq!(g.players.get(&pid).unwrap().money, 200);
    }
}
