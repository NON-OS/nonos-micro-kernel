use crate::image::gif::lzw;
use crate::image::types::{DecodeError, ImageSize};

use super::header::parse_screen;
use super::sub_blocks::gather;
use super::to_argb::{to_argb, Frame};

// Decode the first frame of a GIF87a/89a into ARGB8888. Extensions are walked
// for a Graphic Control Extension (transparency); the first image descriptor
// is LZW-decoded, palette-mapped and composited onto the cleared screen.
pub fn decode_gif_argb8888(input: &[u8], out: &mut [u32]) -> Result<ImageSize, DecodeError> {
    let screen = parse_screen(input)?;
    let sw = screen.size.width as usize;
    let sh = screen.size.height as usize;
    let count = sw.saturating_mul(sh);
    if count > out.len() {
        return Err(DecodeError::OutputTooSmall);
    }
    for p in out.iter_mut().take(count) {
        *p = 0;
    }
    let mut off = screen.blocks_at;
    let mut transparent: Option<u8> = None;
    loop {
        let intro = *input.get(off).ok_or(DecodeError::Truncated)?;
        off += 1;
        match intro {
            0x21 => {
                let label = *input.get(off).ok_or(DecodeError::Truncated)?;
                off += 1;
                let data = gather(input, &mut off)?;
                if label == 0xF9 && data.len() >= 4 && data[0] & 0x01 != 0 {
                    transparent = Some(data[3]);
                }
            }
            0x2C => {
                let d = input.get(off..off + 9).ok_or(DecodeError::Truncated)?;
                let left = u16::from_le_bytes([d[0], d[1]]) as usize;
                let top = u16::from_le_bytes([d[2], d[3]]) as usize;
                let fw = u16::from_le_bytes([d[4], d[5]]) as usize;
                let fh = u16::from_le_bytes([d[6], d[7]]) as usize;
                let packed = d[8];
                off += 9;
                let lct_bytes = if packed & 0x80 != 0 {
                    (1usize << ((packed & 0x07) + 1)).saturating_mul(3)
                } else {
                    0
                };
                let local;
                let palette: &[u8] = if lct_bytes != 0 {
                    local = input.get(off..off + lct_bytes).ok_or(DecodeError::Truncated)?;
                    off += lct_bytes;
                    local
                } else {
                    &screen.gct
                };
                if palette.is_empty() || fw == 0 || fh == 0 {
                    return Err(DecodeError::Unsupported);
                }
                let min_code_size = *input.get(off).ok_or(DecodeError::Truncated)?;
                off += 1;
                let compressed = gather(input, &mut off)?;
                let indices = lzw::decode(min_code_size, &compressed, fw.saturating_mul(fh))?;
                let frame = Frame { w: fw, h: fh, left, top, interlace: packed & 0x40 != 0 };
                to_argb(&indices, palette, transparent, &frame, sw, sh, out)?;
                return Ok(screen.size);
            }
            0x3B => return Err(DecodeError::Unsupported),
            _ => return Err(DecodeError::Unsupported),
        }
    }
}
