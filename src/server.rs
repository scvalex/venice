use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use common::*;
use game::Game;
use data_pack::DataPack;

pub struct Server<'a> {
    games: HashMap<&'a GameId, Game<'a>>,
}

pub enum Event<'a> {
    NewGame(&'a GameId, &'a str),
}

impl<'a> Server<'a> {
    pub fn new() -> Server<'a> {
        Server {
            games: HashMap::new(),
        }
    }

    pub fn handle_event(&mut self, ev: &'a Event) {
        match ev {
            &Event::NewGame(gid, dp) => {
                match self.games.get(gid) {
                    Some(..) => {
                        elog!("game {:?} already exists", gid);
                    }
                    None => {
                        let dp = DataPack::load(&mut File::open(dp).unwrap());
                        let game = Game::new(gid, dp);
                        self.games.insert(gid, game);
                    }
                }
            }
        }
    }
}
