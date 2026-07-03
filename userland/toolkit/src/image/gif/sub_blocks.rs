use alloc::vec::Vec;

use crate::image::types::DecodeError;

// Cap on concatenated sub-block payload so a crafted file cannot exhaust heap.
const MAX_DATA: usize = 32 * 1024 * 1024;

// Concatenate a GIF sub-block chain starting at `*off` (each block is a length
// byte then that many bytes, ending at a zero length). Advances `*off` past
// the terminator.
pub(super) fn gather(input: &[u8], off: &mut usize) -> Result<Vec<u8>, DecodeError> {
    let mut out = Vec::new();
    loop {
        let len = *input.get(*off).ok_or(DecodeError::Truncated)? as usize;
        *off += 1;
        if len == 0 {
            break;
        }
        let data = input.get(*off..*off + len).ok_or(DecodeError::Truncated)?;
        if out.len().saturating_add(len) > MAX_DATA {
            return Err(DecodeError::OutputTooSmall);
        }
        out.extend_from_slice(data);
        *off += len;
    }
    Ok(out)
}
