pub struct Packet {
    reliable: bool,
    sequenced: bool,
    data: Vec<u8>,
}

impl Packet {
    pub fn new(data: Vec<u8>, reliable: bool, sequenced: bool) -> Packet {
        Packet {data, reliable, sequenced}
    }
    pub fn len(&self) -> usize { self.data.len() }
}
