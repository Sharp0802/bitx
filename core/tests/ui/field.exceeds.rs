use bitx::bits;

bits! {
    pub struct OutOfBoundsPacket: 1.0 {
        0.0 pub flag: u1,
        0.1 pub too_large: u16, // Requires 16 bits, exceeds bounds
    }
}

fn main() {}
