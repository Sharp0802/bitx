use bitx::bits;

bits! {
    pub struct InvalidBitPacket: 2.0 {
        0.8 pub flag: u1, // bit must be 0-7
    }
}

fn main() {}
