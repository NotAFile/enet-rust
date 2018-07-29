#![allow(dead_code)]

use bincode;
use serde;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CommandKind {
    Empty(EmptyCommand),
    Ack(AknowledgeCommand),
    Connect(ConnectCommand),
    VerifyConnect(VerifyConnectCommand),
    Disconnect(DisconnectCommand),
    Ping(PingCommand),
    SendReliable(SendReliableCommand),
    SendUnreliable(SendUnreliableCommand),
    SendFragment(SendFragment),
    SendUnsequenced(SendUnsequencedCommand),
    BandwidthLimit(BandwidthLimitCommand),
    ThrottleConfigure(ThrottleConfigureCommand),
    SendUnreliableFragment(SendUnreliableFragmentCommand),
}

impl CommandKind {
    fn get_id(&self) -> u8 {
        // TODO get rid of this disgrace
        match self {
            CommandKind::Empty(c) => c.get_id(),
            CommandKind::Ack(c) => c.get_id(),
            CommandKind::Connect(c) => c.get_id(),
            CommandKind::VerifyConnect(c) => c.get_id(),
            CommandKind::Disconnect(c) => c.get_id(),
            CommandKind::Ping(c) => c.get_id(),
            CommandKind::SendReliable(c) => c.get_id(),
            CommandKind::SendUnreliable(c) => c.get_id(),
            CommandKind::SendFragment(c) => c.get_id(),
            CommandKind::SendUnsequenced(c) => c.get_id(),
            CommandKind::BandwidthLimit(c) => c.get_id(),
            CommandKind::ThrottleConfigure(c) => c.get_id(),
            CommandKind::SendUnreliableFragment(c) => c.get_id(),
        }
    }

    pub fn serialize(&self, reliable_sequence_no: u16, channel_id: u8) -> Vec<u8> {
        let header = CommandHeader {
            command_id: self.get_id(),
            reliable_sequence_no,
            channel_id,
        };
        let mut bytes = bincode::serialize(&header).unwrap();
        bytes.extend(bincode::serialize(&self).unwrap());
        bytes
    }
}

pub trait Command
where
    Self: serde::Serialize,
{
    const ID: u8;

    fn get_id(&self) -> u8 {
        return Self::ID;
    }

    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandHeader {
    channel_id: u8,
    command_id: u8,
    reliable_sequence_no: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyCommand {}

impl Command for EmptyCommand {
    const ID: u8 = 0;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AknowledgeCommand {
    recieved_reliable_sequence_no: u16,
    recieved_sent_time: u16,
}

impl Command for AknowledgeCommand {
    const ID: u8 = 1;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectCommand {
    outgoing_peer_id: u16,
    incoming_session_id: u8,
    outgoing_session_id: u8,
    mtu: u32,
    window_size: u32,
    channel_count: u32,
    incoming_bandwidth: u32,
    outgoing_bandwidth: u32,
    packet_throttle_interval: u32,
    packet_throttle_acceleration: u32,
    packet_throttle_deceleration: u32,
    connection_id: u32,
    initial_data: u32,
}

impl Command for ConnectCommand {
    const ID: u8 = 2;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyConnectCommand {
    outgoing_peer_id: u16,
    incoming_session_id: u8,
    outgoing_session_id: u8,
    mtu: u32,
    window_size: u32,
    channel_count: u32,
    incoming_bandwidth: u32,
    outgoing_bandwidth: u32,
    packet_throttle_interval: u32,
    packet_throttle_acceleration: u32,
    packet_throttle_deceleration: u32,
    connection_id: u32,
}

impl Command for VerifyConnectCommand {
    const ID: u8 = 3;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisconnectCommand {
    final_data: u32,
}

impl Command for DisconnectCommand {
    const ID: u8 = 11;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingCommand {}

impl Command for PingCommand {
    const ID: u8 = 6;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendReliableCommand {
    length: u16,
}

impl Command for SendReliableCommand {
    const ID: u8 = 7;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendUnreliableCommand {
    unreliable_sequence_no: u16,
    length: u16,
}

impl Command for SendUnreliableCommand {
    const ID: u8 = 7;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendFragment {
    unreliable_sequence_no: u16,
    length: u16,
}

impl Command for SendFragment {
    const ID: u8 = 8;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendUnsequencedCommand {
    unsequenced_group: u16,
    data_length: u16,
}

impl Command for SendUnsequencedCommand {
    const ID: u8 = 9;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BandwidthLimitCommand {
    incoming_bandwidth: u32,
    outgoing_bandwidth: u32,
}

impl Command for BandwidthLimitCommand {
    const ID: u8 = 4;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThrottleConfigureCommand {
    packet_throttle_interval: u32,
    packet_throttle_acceleration: u32,
    packet_throttle_deceleration: u32,
}

impl Command for ThrottleConfigureCommand {
    const ID: u8 = 5;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendUnreliableFragmentCommand {
    start_sequence_number: u16,
    length: u16,
    fragment_count: u32,
    fragment_number: u32,
    total_length: u32,
    fragment_offset: u32,
}

impl Command for SendUnreliableFragmentCommand {
    const ID: u8 = 10;
}

#[test]
fn test_command_serialization() {
    println!(
        "{:?}",
        ThrottleConfigureCommand {
            packet_throttle_interval: 12,
            packet_throttle_acceleration: 12,
            packet_throttle_deceleration: 12,
        }.serialize()
    )
}

#[test]
fn test_commandkind_serialization() {
    println!(
        "{:?}",
        CommandKind::ThrottleConfigure(ThrottleConfigureCommand {
            packet_throttle_interval: 12,
            packet_throttle_acceleration: 12,
            packet_throttle_deceleration: 12,
        }).serialize()
    )
}
