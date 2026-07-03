use crate::image::types::{DecodeError, ImageSize};

use super::be_u32::be_u32;
use super::channels::channels;

const PNG_SIG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

// The subset of IHDR layout uses. Grayscale and palette accept bit depths 1,
// 2, 4 and 8; the color formats accept 8. 16-bit and interlaced images fail
// closed as Unsupported.
pub(super) struct Header {
    pub size: ImageSize,
    pub color_type: u8,
    pub channels: usize,
    pub bit_depth: u8,
}

pub(super) fn parse_header(input: &[u8]) -> Result<Header, DecodeError> {
    if input.get(0..8) != Some(&PNG_SIG) {
        return Err(DecodeError::BadMagic);
    }
    if be_u32(input, 8)? != 13 || input.get(12..16) != Some(b"IHDR") {
        return Err(DecodeError::Unsupported);
    }
    let size = ImageSize::new(be_u32(input, 16)?, be_u32(input, 20)?)?;
    let bit_depth = *input.get(24).ok_or(DecodeError::Truncated)?;
    let color_type = *input.get(25).ok_or(DecodeError::Truncated)?;
    let compression = *input.get(26).ok_or(DecodeError::Truncated)?;
    let filter = *input.get(27).ok_or(DecodeError::Truncated)?;
    let interlace = *input.get(28).ok_or(DecodeError::Truncated)?;
    let depth_ok = match color_type {
        0 | 3 => matches!(bit_depth, 1 | 2 | 4 | 8),
        2 | 4 | 6 => bit_depth == 8,
        _ => false,
    };
    if !depth_ok || compression != 0 || filter != 0 || interlace != 0 {
        return Err(DecodeError::Unsupported);
    }
    let channels = channels(color_type)?;
    Ok(Header { size, color_type, channels, bit_depth })
}
