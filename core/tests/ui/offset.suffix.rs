use bitx::bits;

bits! {
    pub struct SuffixPacket: 1.0 {
        0.0u8;1 pub flag, // suffixes are not allowed
    }
}

fn main() {}
