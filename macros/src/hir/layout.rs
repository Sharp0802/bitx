use crate::hir::Mask;
use crate::tt;

#[derive(Debug)]
pub struct Layout {
    pub offset: u32,
    pub size: u32,
    pub aligned: bool,
    pub read_offset_bytes: usize,
    pub read_bytes: usize,
    pub shr: u32,
    pub mask: Option<Mask>,
}

impl From<tt::Layout> for Layout {
    fn from(value: tt::Layout) -> Self {
        let offset = value.offset;
        let size = value.size;

        #[allow(
            clippy::manual_is_multiple_of,
            reason = "it was not stablized at MSRV 1.85"
        )]
        let aligned = offset % 8 == 0 && size % 8 == 0;

        let mut read_offset_bytes = offset / 8;
        let mut read_bytes = (offset % 8 + size).div_ceil(8);

        let shr = read_bytes * 8 - offset % 8 - size;

        let mask = Mask::new(read_bytes * 8);

        if let Some(mask) = &mask {
            let upper = read_offset_bytes + read_bytes;
            let mask_bytes = mask.size / 8;

            if upper >= mask_bytes {
                read_offset_bytes = upper - read_bytes;
                read_bytes = mask_bytes;
            }
        }

        Self {
            offset,
            size,
            aligned,
            read_offset_bytes: read_offset_bytes as usize,
            read_bytes: read_bytes as usize,
            shr,
            mask,
        }
    }
}
