mod compression;
mod channel;
mod packet;
mod host;
mod peer;
pub mod command;

pub use ::peer::*;
pub use ::host::*;
pub use ::packet::*;
pub use ::channel::*;
pub use ::command::Command;

pub const ENET_VERSION: (u8, u8, u8) = (1, 3, 13);

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;

    #[test]
    fn enet_example_server() {
        let client = Host::make_server("127.0.0.1:32887".parse().unwrap());
    }

    #[test]
    fn enet_example_client() {
        let client = Host::make_client();
    }
}
