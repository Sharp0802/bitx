use bitx::bits;

bits! {
    pub enum Overlap: u3 {
        0..=2 => A,
        2..=4 => B, // overlaps with A on value 2
    }
}

fn main() {}
