pub type GameId = String;

pub type PlayerId = String;

pub type UserId = PlayerId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustcDecodable, RustcEncodable, Hash)]
pub struct Quantity(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, RustcDecodable, RustcEncodable)]
pub struct Money(pub u32);

pub type ItemId = String;

pub type AuctionId = String;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Resources {
    pub force: u32,
    pub popularity: u32,
    pub influence: u32,
}

#[macro_export]
macro_rules! elog(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);
