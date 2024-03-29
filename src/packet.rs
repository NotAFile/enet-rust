pub struct Packet<'a> {
    reliable: bool,
    sequenced: bool,
    data: &'a [u8],
}

impl<'a> Packet<'a> {
    pub fn new(data: &'a [u8], reliable: bool, sequenced: bool) -> Packet<'a> {
        Packet {
            reliable,
            sequenced,
            data,
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
