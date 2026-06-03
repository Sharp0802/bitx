use bitx::bits;

bits! {
    pub enum GapBelow: u3 {
        2..=2 => A,
        3..=3 => B,
        4..=4 => C,
    }
}

fn main() {}
