use bitx::bits;

bits! {
    pub struct HugePacket: u130 {
        0.1;129 pub huge_field,
    }
}

fn main() {}
