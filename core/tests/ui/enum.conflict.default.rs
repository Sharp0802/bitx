use bitx::bits;

bits! {
    pub enum ConflictDefault: u3 {
        0..=2 => A,
        _ => B,
        _ => C,
    }
}

fn main() {}
