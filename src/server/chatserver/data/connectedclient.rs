extern crate serde;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ConnectedClient {
    username: String,
    id: u64,
    // Color, Avatar, ...
}
