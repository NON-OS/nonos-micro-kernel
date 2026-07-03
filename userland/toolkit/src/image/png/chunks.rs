use alloc::vec::Vec;

use crate::image::types::DecodeError;

use super::be_u32::be_u32;

// Ceiling on concatenated IDAT so a crafted file cannot exhaust the heap.
const MAX_IDAT: usize = 64 * 1024 * 1024;

// Palette, its transparency, and the concatenated compressed image data.
pub(super) struct Chunks {
    pub idat: Vec<u8>,
    pub plte: Vec<u8>,
    pub trns: Vec<u8>,
}

// Walk the chunk stream after IHDR: concatenate IDAT, capture PLTE and tRNS,
// stop at IEND or the end of input. Every span is bounds-checked.
pub(super) fn gather(input: &[u8]) -> Result<Chunks, DecodeError> {
    let mut out = Chunks { idat: Vec::new(), plte: Vec::new(), trns: Vec::new() };
    let mut off = 8 + 4 + 4 + 13 + 4;
    while off + 8 <= input.len() {
        let clen = be_u32(input, off)? as usize;
        let tag = input.get(off + 4..off + 8).ok_or(DecodeError::Truncated)?;
        let data = input.get(off + 8..off + 8 + clen).ok_or(DecodeError::Truncated)?;
        match tag {
            b"IEND" => break,
            b"IDAT" => {
                if out.idat.len().saturating_add(clen) > MAX_IDAT {
                    return Err(DecodeError::OutputTooSmall);
                }
                out.idat.extend_from_slice(data);
            }
            b"PLTE" => out.plte = data.to_vec(),
            b"tRNS" => out.trns = data.to_vec(),
            _ => {}
        }
        off = off.saturating_add(8).saturating_add(clen).saturating_add(4);
    }
    Ok(out)
}
