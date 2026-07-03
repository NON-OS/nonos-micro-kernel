use crate::image::types::DecodeError;

// Samples per pixel for a PNG color type, which at bit depth 8 is also the
// bytes per pixel. Types: 0 grayscale, 2 truecolor, 3 palette, 4 gray+alpha,
// 6 truecolor+alpha.
pub(super) fn channels(color_type: u8) -> Result<usize, DecodeError> {
    match color_type {
        0 | 3 => Ok(1),
        4 => Ok(2),
        2 => Ok(3),
        6 => Ok(4),
        _ => Err(DecodeError::Unsupported),
    }
}
