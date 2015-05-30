#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GameId(String);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PlayerId(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quantity(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, RustcDecodable, RustcEncodable)]
pub struct Money(u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash, RustcDecodable, RustcEncodable)]
pub struct ItemId(String);

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct AuctionId(String);

#[derive(Debug, RustcDecodable, RustcEncodable)]
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
