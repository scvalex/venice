#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerId(pub String);

pub type UserId = PlayerId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quantity(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, RustcDecodable, RustcEncodable)]
pub struct Money(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash, RustcDecodable, RustcEncodable)]
pub struct ItemId(pub String);

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct AuctionId(String);

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
