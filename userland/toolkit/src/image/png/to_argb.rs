use crate::image::types::DecodeError;

use super::argb_bytes::argb_bytes;
use super::argb_packed::argb_packed;
use super::header::Header;

// Expand unfiltered scanlines into ARGB8888. Byte-aligned (8-bit) formats and
// sub-byte packed grayscale/palette take separate paths; `row_bytes` is the
// packed length of one row so both index the buffer the same way.
pub(super) fn to_argb(
    rows: &[u8],
    h: &Header,
    row_bytes: usize,
    plte: &[u8],
    trns: &[u8],
    out: &mut [u32],
) -> Result<(), DecodeError> {
    let w = h.size.width as usize;
    let ht = h.size.height as usize;
    if out.len() < w.saturating_mul(ht) {
        return Err(DecodeError::OutputTooSmall);
    }
    if h.bit_depth == 8 {
        argb_bytes(rows, h, row_bytes, plte, trns, out)
    } else {
        argb_packed(rows, h, row_bytes, plte, trns, out)
    }
}
