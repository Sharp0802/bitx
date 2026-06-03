use bitx::bits;

bits! {
    pub enum GapAbove: u3 {
        0..=0 => A,
        1..=1 => B,
        2..=2 => C,
        // values 3, 4, 5, 6, 7 are uncovered
    }
}

fn main() {}
