use alloc::vec;
use alloc::vec::Vec;

use crate::image::types::DecodeError;

const MAX_CODES: usize = 4096;

// Decode GIF variable-width LZW into palette indices. `min_code_size` sets the
// initial code width; codes grow as the dictionary fills and reset on the
// clear code. Output is capped at `max_out` so a stream cannot overrun the
// frame. Truncated input returns what was decoded so far.
pub(super) fn decode(
    min_code_size: u8,
    data: &[u8],
    max_out: usize,
) -> Result<Vec<u8>, DecodeError> {
    if !(2..=8).contains(&min_code_size) {
        return Err(DecodeError::Unsupported);
    }
    let clear = 1u16 << min_code_size;
    let end = clear + 1;
    let mut prefix = vec![0u16; MAX_CODES];
    let mut suffix = vec![0u8; MAX_CODES];
    for i in 0..clear as usize {
        suffix[i] = i as u8;
    }
    let mut code_size = min_code_size + 1;
    let mut free = clear + 2;
    let mut old: i32 = -1;
    let mut first: u8 = 0;
    let mut out: Vec<u8> = Vec::new();
    let mut stack: Vec<u8> = Vec::new();
    let mut bitbuf: u32 = 0;
    let mut bits: u32 = 0;
    let mut pos = 0usize;
    loop {
        while bits < code_size as u32 {
            let Some(&byte) = data.get(pos) else {
                return Ok(out);
            };
            bitbuf |= (byte as u32) << bits;
            bits += 8;
            pos += 1;
        }
        let code = (bitbuf & ((1u32 << code_size) - 1)) as u16;
        bitbuf >>= code_size;
        bits -= code_size as u32;
        if code == clear {
            code_size = min_code_size + 1;
            free = clear + 2;
            old = -1;
            continue;
        }
        if code == end {
            break;
        }
        if old < 0 {
            first = suffix[code as usize];
            out.push(first);
            old = code as i32;
            continue;
        }
        let mut cur = code;
        if code >= free {
            stack.push(first);
            cur = old as u16;
        }
        while cur >= clear {
            let c = cur as usize;
            if c >= MAX_CODES || stack.len() > MAX_CODES {
                return Err(DecodeError::Unsupported);
            }
            stack.push(suffix[c]);
            cur = prefix[c];
        }
        first = suffix[cur as usize];
        stack.push(first);
        while let Some(b) = stack.pop() {
            out.push(b);
            if out.len() >= max_out {
                return Ok(out);
            }
        }
        if free < MAX_CODES as u16 {
            prefix[free as usize] = old as u16;
            suffix[free as usize] = first;
            free += 1;
            if free == (1u16 << code_size) && code_size < 12 {
                code_size += 1;
            }
        }
        old = code as i32;
    }
    Ok(out)
}
