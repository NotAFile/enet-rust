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

pub use crate::channel::*;
pub use crate::command::Command;
pub use crate::host::*;
pub use crate::packet::*;
pub use crate::peer::*;

pub const ENET_VERSION: (u8, u8, u8) = (1, 3, 13);

#[cfg(test)]
mod tests {

    #[test]
    fn enet_example_server() {
        // let server = Host::new("127.0.0.1:32887");
    }

    #[test]
    fn enet_example_client() {
        // let client = Host::new("127.0.0.1:32887");
    }
}
