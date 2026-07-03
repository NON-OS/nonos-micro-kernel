use alloc::vec::Vec;

use crate::image::types::{DecodeError, ImageSize};

// Logical screen: canvas size, the global color table (RGB triples, empty when
// absent) and the byte offset where the block stream begins.
pub(super) struct Screen {
    pub size: ImageSize,
    pub gct: Vec<u8>,
    pub blocks_at: usize,
}

pub(super) fn parse_screen(input: &[u8]) -> Result<Screen, DecodeError> {
    let sig = input.get(0..6).ok_or(DecodeError::Truncated)?;
    if sig != b"GIF87a" && sig != b"GIF89a" {
        return Err(DecodeError::BadMagic);
    }
    let width = u16::from_le_bytes([
        *input.get(6).ok_or(DecodeError::Truncated)?,
        *input.get(7).ok_or(DecodeError::Truncated)?,
    ]);
    let height = u16::from_le_bytes([
        *input.get(8).ok_or(DecodeError::Truncated)?,
        *input.get(9).ok_or(DecodeError::Truncated)?,
    ]);
    let size = ImageSize::new(width as u32, height as u32)?;
    let packed = *input.get(10).ok_or(DecodeError::Truncated)?;
    let gct = if packed & 0x80 != 0 {
        let entries = 1usize << ((packed & 0x07) + 1);
        let bytes = entries.saturating_mul(3);
        input.get(13..13 + bytes).ok_or(DecodeError::Truncated)?.to_vec()
    } else {
        Vec::new()
    };
    let blocks_at = 13 + gct.len();
    Ok(Screen { size, gct, blocks_at })
}
