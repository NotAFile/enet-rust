use byteorder::{ByteOrder, ReadBytesExt, BE};
use enet::command::{ConnectCommand, ProtocolHeader};
use std::error::Error;
use std::io::Cursor;
use std::net::UdpSocket;

pub fn to_hex_string(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join(" ")
}

fn main() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:32887")?;

    let mut bincode_ser = bincode::config();
    bincode_ser.big_endian();

    loop {
        let mut buf = [0; 1500];
        let (amt, src) = socket.recv_from(&mut buf)?;

        let buf = &mut buf[..amt];
        println!("IN {} ({})", src, amt);

        let mut header_rdr = Cursor::new(&buf[..8]);

        let header: ProtocolHeader = bincode_ser.deserialize(&buf[..8])?;
        println!("      HDR: {}", to_hex_string(&buf[..8]));
        let peerID = header_rdr.read_u16::<BE>()?;
        println!(
            "           Flags: {:b} (time, compress)",
            peerID >> (16 - 2)
        );
        println!("           SessionID: {}", (peerID >> 12) & 0b11);
        println!("           PeerID: {:x}", peerID & 0x0FFF);
        println!("           Time: {}", header_rdr.read_u16::<BE>()?);
        let commandID = header_rdr.read_u8()?;
        println!("           Command Flags: {:b}", commandID >> 4);
        println!("           Command ID: {}", commandID & 0x0F);
        println!("           ChannelID: {}", header_rdr.read_u8()?);
        println!(
            "           ReliableSequenceNo: {}",
            header_rdr.read_u16::<BE>()?
        );
        println!("      CMD: {}", to_hex_string(&buf[8..]));
        let connect: ConnectCommand = bincode_ser.deserialize(&buf[8..])?;
        println!("{:?}", connect);
    }

    // let result: CommandKind = bincode::deserialize(buf)?;

    // println!("{:?}", result);

    Ok(())
}
