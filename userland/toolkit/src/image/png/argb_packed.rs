use crate::image::types::DecodeError;

use super::header::Header;

// Sub-byte conversion for grayscale and palette at bit depth 1/2/4: samples
// pack MSB-first within each byte. Grayscale scales to the full 0..255 range;
// palette indices resolve through PLTE with tRNS alpha.
pub(super) fn argb_packed(
    rows: &[u8],
    h: &Header,
    row_bytes: usize,
    plte: &[u8],
    trns: &[u8],
    out: &mut [u32],
) -> Result<(), DecodeError> {
    let w = h.size.width as usize;
    let ht = h.size.height as usize;
    let d = h.bit_depth as usize;
    let maxval = ((1u32 << d) - 1).max(1);
    let mask = ((1u16 << d) - 1) as u16;
    for y in 0..ht {
        let base = y * row_bytes;
        for x in 0..w {
            let bit = x * d;
            let byte = *rows.get(base + bit / 8).ok_or(DecodeError::Truncated)?;
            let shift = 8 - d - (bit % 8);
            let sample = ((byte as u16) >> shift) & mask;
            let (r, g, b, a) = match h.color_type {
                0 => {
                    let v = (sample as u32 * 255 / maxval) as u8;
                    (v, v, v, 255)
                }
                3 => {
                    let idx = sample as usize;
                    let e = idx.saturating_mul(3);
                    let rgb = plte.get(e..e + 3).ok_or(DecodeError::Unsupported)?;
                    (rgb[0], rgb[1], rgb[2], trns.get(idx).copied().unwrap_or(255))
                }
                _ => return Err(DecodeError::Unsupported),
            };
            out[y * w + x] =
                ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        }
    }
    Ok(())
}
