// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use alloc::vec;
use alloc::vec::Vec;

use nonos_toolkit::image::jpeg::decode_jpeg_argb8888;
use nonos_toolkit::types::ImageSize;

const MAX_PIXELS: usize = 1920 * 1080;
const MAX_DIM: u32 = 4096;
const MIN_JPEG: usize = 4;

pub struct DecodedImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>,
}

pub fn decode_jpeg(input: &[u8]) -> Option<DecodedImage> {
    if input.len() < MIN_JPEG {
        return None;
    }
    if input[0] != 0xFF || input[1] != 0xD8 {
        return None;
    }
    let mut pixels: Vec<u32> = vec![0u32; MAX_PIXELS];
    let size: ImageSize = decode_jpeg_argb8888(input, &mut pixels).ok()?;
    if size.width == 0 || size.height == 0 || size.width > MAX_DIM || size.height > MAX_DIM {
        return None;
    }
    let count = (size.width as usize).checked_mul(size.height as usize)?;
    if count > MAX_PIXELS {
        return None;
    }
    pixels.truncate(count);
    Some(DecodedImage { width: size.width, height: size.height, pixels })
}
