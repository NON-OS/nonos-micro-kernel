use crate::image::types::DecodeError;

use super::paeth::paeth;

// Reverse PNG scanline filters into `out`, which receives height * row_bytes
// reconstructed bytes (no per-row filter byte). `bpp` is the filter unit: the
// bytes per pixel rounded up to at least one, so left/up-left references land
// on the right byte for every format including sub-byte packings.
pub fn unfilter(
    raw: &[u8],
    row_bytes: usize,
    height: usize,
    bpp: usize,
    out: &mut [u8],
) -> Result<usize, DecodeError> {
    let needed = row_bytes.saturating_add(1).saturating_mul(height);
    if raw.len() < needed {
        return Err(DecodeError::Truncated);
    }
    let out_needed = row_bytes.saturating_mul(height);
    if out.len() < out_needed {
        return Err(DecodeError::OutputTooSmall);
    }
    let mut src = 0usize;
    for row in 0..height {
        let filter = raw[src];
        src += 1;
        let base = row * row_bytes;
        for i in 0..row_bytes {
            let x = raw[src + i] as i32;
            let a = if i >= bpp { out[base + i - bpp] as i32 } else { 0 };
            let b = if row > 0 { out[base - row_bytes + i] as i32 } else { 0 };
            let c = if row > 0 && i >= bpp { out[base - row_bytes + i - bpp] as i32 } else { 0 };
            let recon = match filter {
                0 => x,
                1 => x + a,
                2 => x + b,
                3 => x + (a + b) / 2,
                4 => x + paeth(a, b, c),
                _ => return Err(DecodeError::Unsupported),
            };
            out[base + i] = (recon & 0xff) as u8;
        }
        src += row_bytes;
    }
    Ok(out_needed)
}
