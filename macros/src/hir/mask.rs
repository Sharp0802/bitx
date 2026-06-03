use crate::tt::Type;

#[derive(Clone)]
pub struct Mask {
    pub size: u32,
    pub ty: Type,
}

impl Mask {
    pub fn new(size: u32) -> Option<Self> {
        let size = size.div_ceil(8).next_power_of_two() * 8;
        if (1..=128).contains(&size) {
            let ty = Type::literal(size);
            Some(Self { size, ty })
        } else {
            None
        }
    }
}
