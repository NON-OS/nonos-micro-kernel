use crate::image::types::{DecodeError, ImageSize};

fn le_u16(bytes: &[u8], off: usize) -> Result<u16, DecodeError> {
    let b = bytes.get(off..off + 2).ok_or(DecodeError::Truncated)?;
    Ok(u16::from_le_bytes([b[0], b[1]]))
}

fn le_u32(bytes: &[u8], off: usize) -> Result<u32, DecodeError> {
    let b = bytes.get(off..off + 4).ok_or(DecodeError::Truncated)?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

pub fn bmp_dimensions(input: &[u8]) -> Result<ImageSize, DecodeError> {
    if input.get(0..2) != Some(b"BM") {
        return Err(DecodeError::BadMagic);
    }
    ImageSize::new(le_u32(input, 18)?, le_u32(input, 22)?)
}

pub fn decode_bmp_argb8888(input: &[u8], out: &mut [u32]) -> Result<ImageSize, DecodeError> {
    if input.get(0..2) != Some(b"BM") {
        return Err(DecodeError::BadMagic);
    }
    let data_off = le_u32(input, 10)? as usize;
    let dib_size = le_u32(input, 14)?;
    if dib_size < 40 {
        return Err(DecodeError::Unsupported);
    }
    let width = le_u32(input, 18)?;
    let height = le_u32(input, 22)?;
    let planes = le_u16(input, 26)?;
    let bpp = le_u16(input, 28)?;
    let compression = le_u32(input, 30)?;
    if planes != 1 || bpp != 32 || compression != 0 {
        return Err(DecodeError::Unsupported);
    }
    let size = ImageSize::new(width, height)?;
    let count = size.pixel_count() as usize;
    if count > out.len() {
        return Err(DecodeError::OutputTooSmall);
    }
    let row_bytes = width as usize * 4;
    let need = data_off.saturating_add(row_bytes.saturating_mul(height as usize));
    if need > input.len() {
        return Err(DecodeError::Truncated);
    }
    let mut y = 0usize;
    while y < height as usize {
        let src_row = height as usize - 1 - y;
        let src_off = data_off + src_row * row_bytes;
        let dst_off = y * width as usize;
        let mut x = 0usize;
        while x < width as usize {
            let i = src_off + x * 4;
            let b = input[i] as u32;
            let g = input[i + 1] as u32;
            let r = input[i + 2] as u32;
            let a = input[i + 3] as u32;
            out[dst_off + x] = (a << 24) | (r << 16) | (g << 8) | b;
            x += 1;
        }
        y += 1;
    }
    Ok(size)
}
