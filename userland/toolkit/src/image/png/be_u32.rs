use crate::image::types::DecodeError;

// Big-endian u32 at `off`, or Truncated if the four bytes are not present.
pub(super) fn be_u32(input: &[u8], off: usize) -> Result<u32, DecodeError> {
    let b = input.get(off..off + 4).ok_or(DecodeError::Truncated)?;
    Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}
