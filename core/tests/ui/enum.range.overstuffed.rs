use bitx::bits;

bits! {
    pub enum RangeOverMax: u2 {
        0..=4 => A, // upper bound 4 exceeds max 3 for u2
    }
}

fn main() {}
