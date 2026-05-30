use bitx::bits;

bits! {
    pub struct SuffixPacket: 1.0 {
        0usize pub flag: u1, // suffixes are not allowed
    }
}

fn main() {}
