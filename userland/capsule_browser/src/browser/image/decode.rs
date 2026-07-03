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

use nonos_toolkit::image::{bmp, gif, jpeg, png};

use super::sniff::{probe, Format};
use super::store::Decoded;

// Refuse rasters beyond this many pixels rather than allocate a huge buffer.
const MAX_PIXELS: u64 = 4_000_000;

// Decode raw image bytes (PNG/JPEG/BMP/GIF/SVG) into an ARGB8888 raster.
pub(super) fn decode_body(bytes: &[u8]) -> Result<Decoded, &'static str> {
    if super::svg::is_svg(bytes) {
        return super::svg::decode_svg(bytes).ok_or("svg parse failed");
    }
    let p = probe(bytes).ok_or("unknown image format")?;
    if p.w == 0 || p.h == 0 {
        return Err("bad image dimensions");
    }
    if p.w as u64 * p.h as u64 > MAX_PIXELS {
        return Err("image too large");
    }
    let mut px = vec![0u32; p.w as usize * p.h as usize];
    let size = match p.format {
        Format::Png => png::decoder::decode_png_argb8888(bytes, &mut px),
        Format::Jpeg => jpeg::decode_jpeg_argb8888(bytes, &mut px),
        Format::Bmp => bmp::decode_bmp_argb8888(bytes, &mut px),
        Format::Gif => gif::decode_gif_argb8888(bytes, &mut px),
    }
    .map_err(|_| "image decode failed")?;
    Ok(Decoded { w: size.width, h: size.height, px })
}
