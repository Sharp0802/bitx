use crate::prelude::*;

pub struct Mask {
    pub size: Offset,
    pub ty: Type,
}

impl Mask {
    pub fn for_size(size: Offset) -> Option<Self> {
        let size =
            Offset::from_bytes(size.bits().div_ceil(8).next_power_of_two());
        if size.bits() < 1 || 128 < size.bits() {
            return None;
        }

        let ty = lit::with_size(size);
        Some(Self { size, ty })
    }
}
