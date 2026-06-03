use bitx::bits;

bits! {
    pub enum MidGap: u4 {
        0..=1 => A,
        // values 2, 3 are uncovered
        4..=7 => B,
    }
}

fn main() {}
