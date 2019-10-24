extern crate enet;
use std::net::{SocketAddr, UdpSocket};

fn main() -> Result<(), Err> {
    let mut socket = UdpSocket::bind("0.0.0.0")?;
    let initial_data = 42;
    let addr = "127.0.0.1:32887".parse();
    let peer = host.connect(addr.unwrap(), initial_data, 3);

    while peer.in_transit() {
        host.service();
    }

    // peer.send_reliable(0, b"reliable");
    // peer.send_sequenced(0, b"sequenced");
    // peer.send_unreliable(0, b"unsequenced");

}
