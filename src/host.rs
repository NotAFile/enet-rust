use std::net::{UdpSocket, SocketAddr};
use std::io::Error;
use super::peer::Peer;

const DEFAULT_MTU: usize = 1400;

pub struct Host <'a> {
    socket: UdpSocket,
    // bandwidth limitation unimplemented
    mtu: usize,
    random_seed: u32,
    peers: Vec<Peer<'a>>,
    // channel limit unimplemented
}

impl <'a> Host <'a> {
    fn new(address: Option<SocketAddr>) -> Result<Host<'a>, Error> {
        let socket = match address {
            Some(addr) => UdpSocket::bind(addr)?,
            None => UdpSocket::bind("0.0.0.0:0")? // this should let the OS choose
        };
        return Ok(Host {
            socket, 
            mtu: DEFAULT_MTU,
            random_seed: 0,
            peers: Vec::new(),
        })
    }

    pub fn make_client() -> Result<Host<'a>, Error> {
        Host::new(None) // TODO: some way of getting the current struct?
    }

    pub fn make_server(address: SocketAddr) -> Result<Host<'a>, Error> {
        Host::new(Some(address)) // TODO: some way of getting the current struct?
    }

    pub fn connect(&mut self, address: SocketAddr, channel_count: u8, connect_data: u32) {
        let peer = Peer::new(address, self);
        self.peers.push(peer);
    }
    
    pub fn mtu(&self) -> usize { self.mtu }
}
