use common::*;

pub enum Event<'a> {
    JoinGame(&'a GameId, &'a PlayerId),
}
