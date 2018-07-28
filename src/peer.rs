#![allow(dead_code)]
use std::collections::VecDeque;
use std::io::Error;
use std::net::SocketAddr;

use super::{Channel, Command, Host, Packet};
use command;
use command::CommandKind;

enum ConnectionState {
    Disconnected,
    Connecting,
    AckConnect,
    ConnectPend,
    ConnectSuccess,
    Connected,
    DisconnectLater,
    Disconnecting,
    AckDisconnect,
    Zombie,
}

pub struct Peer<'a> {
    channels: Vec<Channel>,
    state: ConnectionState,
    address: SocketAddr,
    mtu: usize,
    pub outgoing_commands: VecDeque<&'a CommandKind>,
    pub incoming_commands: VecDeque<&'a CommandKind>,
}

impl<'a> Peer<'a> {
    pub fn new(address: SocketAddr, host: &Host) -> Peer<'a> {
        Peer {
            address,
            channels: Vec::new(),
            state: ConnectionState::Disconnected,
            mtu: host.mtu(),
            outgoing_commands: VecDeque::new(),
            incoming_commands: VecDeque::new(),
        }
    }

    // TODO: modify usize to u8 and figure out how to compare them
    pub fn send(&self, channel_id: u8, packet: Packet) -> Result<(), Error> {
        use self::ConnectionState::*;
        if let Connected = self.state {
            if channel_id as usize > self.channels.len() {
                panic!(
                    "Tried to send on channel {}, but peer only has {} channels",
                    channel_id,
                    self.channels.len()
                )
            }
            // TODO: Determine of packet size is too large

            let frag_length = self.mtu; // TODO: - sizeof (EnetProtocolHeader, EnetProtocolSendFragment)
                                        // TODO: subtract checksum length from frag_length, if it is enabled

            if packet.len() > frag_length {
                // Enet packet does not fit within one UDP packet, we need to fragment
                let frag_count = packet.len() + frag_length - 1;

                // TODO: check max fragment count
            }
            Ok(())
        } else {
            panic!("not connected");
        }
    }

    pub fn ping(&mut self) {
        self.outgoing_commands
            .push_back(&CommandKind::Ping(command::PingCommand {}));
    }
}
