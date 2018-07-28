#![allow(dead_code)]
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde;

mod channel;
pub mod command;
mod compression;
mod host;
mod packet;
mod peer;

pub use channel::*;
pub use command::Command;
pub use host::*;
pub use packet::*;
pub use peer::*;

pub const ENET_VERSION: (u8, u8, u8) = (1, 3, 13);

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, SocketAddr};

    #[test]
    fn enet_example_server() {
        // let server = Host::new("127.0.0.1:32887");
    }

    #[test]
    fn enet_example_client() {
        // let client = Host::new("127.0.0.1:32887");
    }
}
