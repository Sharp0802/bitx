use bitx::bits;

// A dummy type mimicking a large struct
pub struct LargeData([u8; 32]);

impl bitx::Bits for LargeData {
    type Mask = ();
    type Read<'a> = &'a LargeData;
    const BITS: u32 = 256;
}

bits! {
    pub struct HugePacket: u257 {
        0.1;256 pub huge_field: LargeData,
        // unaligned extraction > 128 bits
    }
}

fn main() {}
