pub enum Command {
    Ack(AknowledgeCommand),
    Connect(ConnectCommand),
    VerifyConnect(VerifyConnectCommand),
    Disconnect(DisconnectCommand),
    Ping(PingCommand),
    SendReliable(SendReliableCommand),
    SendUnreliable(SendUnreliableCommand),
    SendUnsequenced(SendUnsequencedCommand),
    SendFragment(SendFragmentCommand),
    BandwidthLimit(BandwidthLimitCommand),
    ThrottleConfigure(ThrottleConfigureCommand),
}

pub struct CommandHeader {
    command: u8,
    channel_id: u8,
    reliable_sequence_no: u16,
}

pub struct AknowledgeCommand {
    header: CommandHeader,
    recieved_reliable_sequence_no: u16,
    recieved_sent_time: u16,
}

pub struct ConnectCommand {
    header: CommandHeader,
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

pub struct VerifyConnectCommand {
    header: CommandHeader,
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

pub struct BandwidthLimitCommand {
    header: CommandHeader,
    incoming_bandwidth: u32,
    outgoing_bandwidth: u32,
}

pub struct ThrottleConfigureCommand {
    header: CommandHeader,
    packet_throttle_interval: u32,
    packet_throttle_acceleration: u32,
    packet_throttle_deceleration: u32,
}

pub struct PingCommand {
    header: CommandHeader,
}

pub struct SendReliableCommand {
    header: CommandHeader,
    length: u16,
}

pub struct SendUnreliableCommand {
    header: CommandHeader,
    unreliable_sequence_no: u16,
    length: u16,
}

pub struct SendUnsequencedCommand {
    header: CommandHeader,
    unsequenced_group: u16,
    data_length: u16,
}

pub struct SendFragmentCommand {
    header: CommandHeader,
    start_sequence_number: u16,
    length: u16,
    fragment_count: u32,
    fragment_number: u32,
    total_length: u32,
    fragment_offset: u32,
}

pub struct DisconnectCommand {
    header: CommandHeader,
    final_data: u32,
}
