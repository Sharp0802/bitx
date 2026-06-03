use bitx::bits;

bits! {
    pub struct InvalidBitPacket: u16 {
        0.8;1 pub flag: u1, // bit must be 0-7
    }
}

fn main() {}
