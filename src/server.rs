use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use common::*;
use game::Game;
use data_pack::DataPack;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct User {
    pub id: UserId,
    password_hash: String,
}

fn hash_password(pass: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(pass);
    hasher.result_str()
}

impl User {
    pub fn new(uid: UserId, pass: &str) -> User {
        User {
            id: uid,
            password_hash: hash_password(pass),
        }
    }
}

pub enum Event<'a> {
    NewGame(GameId, &'a str),
    NewUser(UserId, &'a str),
}

pub struct Server {
    games: HashMap<GameId, Game>,
    users: HashMap<UserId, User>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            games: HashMap::new(),
            users: HashMap::new(),
        }
    }

    pub fn handle_event(&mut self, ev: Event) {
        match ev {
            Event::NewGame(ref gid, dp) => {
                match self.games.get(gid) {
                    Some(..) => {
                        elog!("game {:?} already exists", gid);
                    }
                    None => {
                        let dp = DataPack::load(&mut File::open(dp).unwrap());
                        let game = Game::new(gid.clone(), dp);
                        self.games.insert(gid.clone(), game);
                    }
                }
            }
            Event::NewUser(ref uid, pass) => {
                match self.users.get(uid) {
                    Some(..) => {
                        elog!("user {:?} already exists", uid);
                    }
                    None => {
                        self.users.insert(uid.clone(), User::new(uid.clone(), pass));
                    }
                }
            }
        }
    }

    pub fn users(&self) -> HashSet<&User> {
        let mut users = HashSet::new();
        self.users.values().inspect(|user| {
            users.insert(user.clone());
        });
        users
    }
}
