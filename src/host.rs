use super::peer::Peer;
use std::io::Error;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::rc::Rc;

const DEFAULT_MTU: usize = 1400;

pub struct Host<'a> {
    socket: UdpSocket,
    // bandwidth limitation unimplemented
    mtu: usize,
    random_seed: u32,
    peers: Vec<Rc<Peer<'a>>>,
    // channel limit unimplemented
}

impl<'a> Host<'a> {
    pub fn new<A: ToSocketAddrs>(address: Option<A>) -> Result<Host<'a>, Error> {
        let socket = UdpSocket::bind(address.unwrap())?;
        return Ok(Host {
            socket,
            mtu: DEFAULT_MTU,
            random_seed: 0,
            peers: Vec::new(),
        });
    }

    pub fn connect(
        &mut self,
        address: SocketAddr,
        channel_count: u8,
        connect_data: u32,
    ) -> Rc<Peer> {
        let peer = Rc::new(Peer::new(address, self));
        self.peers.push(peer.clone());
        peer
    }

    pub fn send_queued_packets(&mut self, mut peer: Peer) -> Result<(), Error> {
        let command = peer.outgoing_commands.pop_front().unwrap();
        self.socket.send(command.serialize(0, 0xFF).as_slice())?;
        Ok(())
    }

    pub fn service() {}

    pub fn mtu(&self) -> usize {
        self.mtu
    }
}
