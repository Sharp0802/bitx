use bitx::bits;

bits! {
    pub struct OutOfBoundsPacket: u8 {
        0.0;01 pub flag,
        0.1;16 pub too_large, // Requires 16 bits, exceeds bounds
    }
}

fn main() {}
