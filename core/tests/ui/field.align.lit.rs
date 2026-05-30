use bitx::bits;

bits! {
    pub struct HugePacket: 33.0 {
        0.1 pub huge_field: u129,
        // unaligned extraction > 128 bits
    }
}

fn main() {}
