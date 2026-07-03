use crate::image::types::DecodeError;

use super::deinterlaced_row::deinterlaced_row;

// One frame's geometry within the logical screen.
pub(super) struct Frame {
    pub w: usize,
    pub h: usize,
    pub left: usize,
    pub top: usize,
    pub interlace: bool,
}

// Composite palette indices onto the ARGB8888 screen at the frame's offset.
// `out` is pre-cleared to transparent, so transparent-index pixels are simply
// left untouched. Interlaced frames map through the pass order.
pub(super) fn to_argb(
    indices: &[u8],
    palette: &[u8],
    transparent: Option<u8>,
    f: &Frame,
    screen_w: usize,
    screen_h: usize,
    out: &mut [u32],
) -> Result<(), DecodeError> {
    for row in 0..f.h {
        let fy = if f.interlace {
            deinterlaced_row(row, f.h).ok_or(DecodeError::Truncated)?
        } else {
            row
        };
        let dst_y = f.top + fy;
        if dst_y >= screen_h {
            continue;
        }
        for x in 0..f.w {
            let idx = *indices.get(row * f.w + x).ok_or(DecodeError::Truncated)?;
            if transparent == Some(idx) {
                continue;
            }
            let dst_x = f.left + x;
            if dst_x >= screen_w {
                continue;
            }
            let e = (idx as usize).saturating_mul(3);
            let rgb = palette.get(e..e + 3).ok_or(DecodeError::Unsupported)?;
            out[dst_y * screen_w + dst_x] =
                (255u32 << 24) | ((rgb[0] as u32) << 16) | ((rgb[1] as u32) << 8) | rgb[2] as u32;
        }
    }
    Ok(())
}
