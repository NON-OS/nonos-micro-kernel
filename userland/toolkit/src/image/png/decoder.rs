use alloc::vec;

use crate::image::png::inflate::inflate_zlib;
use crate::image::png::scanline::unfilter;
use crate::image::types::{DecodeError, ImageSize};

use super::chunks::gather;
use super::header::parse_header;
use super::to_argb::to_argb;

// Ceiling on the inflated and unfiltered staging buffers, so a small file that
// claims huge dimensions cannot force a runaway allocation.
const MAX_STAGE: usize = 64 * 1024 * 1024;

// Decode a non-interlaced PNG (grayscale, truecolor, palette, gray+alpha or
// truecolor+alpha; bit depths 1/2/4/8 for grayscale and palette, 8 for the
// color formats) into ARGB8888. `out` must hold at least width*height pixels.
pub fn decode_png_argb8888(input: &[u8], out: &mut [u32]) -> Result<ImageSize, DecodeError> {
    let h = parse_header(input)?;
    let w = h.size.width as usize;
    let ht = h.size.height as usize;
    if w.saturating_mul(ht) > out.len() {
        return Err(DecodeError::OutputTooSmall);
    }
    // Bits per pixel, then packed row length and the filter unit (bytes per
    // pixel, at least one) that the reconstruction references.
    let bits_pp = h.channels.saturating_mul(h.bit_depth as usize);
    let row_bytes = w.saturating_mul(bits_pp).saturating_add(7) / 8;
    let bpp = bits_pp.saturating_add(7) / 8;
    let raw_len = row_bytes.saturating_add(1).saturating_mul(ht);
    let unfiltered_len = row_bytes.saturating_mul(ht);
    if raw_len == 0 || raw_len > MAX_STAGE || unfiltered_len > MAX_STAGE {
        return Err(DecodeError::Unsupported);
    }
    let chunks = gather(input)?;
    if chunks.idat.is_empty() {
        return Err(DecodeError::Unsupported);
    }
    let mut deflated = vec![0u8; raw_len];
    let n = inflate_zlib(&chunks.idat, &mut deflated)?;
    let mut rows = vec![0u8; unfiltered_len];
    let got = unfilter(&deflated[..n], row_bytes, ht, bpp, &mut rows)?;
    if got != unfiltered_len {
        return Err(DecodeError::Truncated);
    }
    to_argb(&rows, &h, row_bytes, &chunks.plte, &chunks.trns, out)?;
    Ok(h.size)
}
