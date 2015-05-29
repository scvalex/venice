#[derive(Debug, PartialEq, Eq)]
pub struct GameId(String);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PlayerId(String);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Quantity(u32);

#[derive(Debug, PartialEq, Eq, Hash, RustcDecodable, RustcEncodable)]
pub struct Money(u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash, RustcDecodable, RustcEncodable)]
pub struct ItemId(String);

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct AuctionId(String);
