use crate::image::types::DecodeError;

use super::header::Header;

// Byte-aligned 8-bit conversion: one byte per sample. Palette indices resolve
// through PLTE, with tRNS supplying per-index alpha (opaque where absent).
pub(super) fn argb_bytes(
    rows: &[u8],
    h: &Header,
    row_bytes: usize,
    plte: &[u8],
    trns: &[u8],
    out: &mut [u32],
) -> Result<(), DecodeError> {
    let w = h.size.width as usize;
    let ht = h.size.height as usize;
    let ch = h.channels;
    for y in 0..ht {
        let base = y * row_bytes;
        for x in 0..w {
            let p = base + x * ch;
            let px = rows.get(p..p + ch).ok_or(DecodeError::Truncated)?;
            let (r, g, b, a) = match h.color_type {
                6 => (px[0], px[1], px[2], px[3]),
                2 => (px[0], px[1], px[2], 255),
                0 => (px[0], px[0], px[0], 255),
                4 => (px[0], px[0], px[0], px[1]),
                3 => {
                    let idx = px[0] as usize;
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
