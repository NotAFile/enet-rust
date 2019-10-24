use super::peer::Peer;
use std::io::Error;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::rc::Rc;

const DEFAULT_MTU: usize = 1400;

#[derive(Debug, Copy, Clone)]
pub struct HostConfig {
    pub mtu: usize,
}

pub struct Host {
    socket: UdpSocket,
    // bandwidth limitation unimplemented
    config: HostConfig,
    random_seed: u32,
    peers: Vec<Rc<Peer>>,
    // channel limit unimplemented
}

impl Host {
    pub fn new<A: ToSocketAddrs>(address: A) -> Result<Host, Error> {
        let socket = UdpSocket::bind(address)?;
        return Ok(Host {
            socket,
            config: HostConfig {
                mtu: DEFAULT_MTU,
            },
            random_seed: 0,
            peers: Vec::new(),
        });
    }

    pub fn connect(
        &mut self,
        address: SocketAddr,
        _channel_count: u8,
        _connect_data: u32,
    ) -> Rc<Peer> {
        let peer = Rc::new(Peer::new(address, self.config));
        self.peers.push(peer);
        self.peers.last().unwrap().clone()
    }

    pub fn send_queued_packets(&mut self, mut peer: Peer) -> Result<(), Error> {
        let command = peer.outgoing_commands.pop_front().unwrap();
        self.socket.send(command.serialize(0, 0xFF).as_slice())?;
        Ok(())
    }

    pub fn service(&mut self) -> Result<(), Error> {
        let mut buf = [0; 1024];
        let (amt, src) = self.socket.recv_from(&mut buf)?;

        println!("{:?}: {:?}", amt, src);
        Ok(())
    }
}
