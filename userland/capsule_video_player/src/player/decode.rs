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

use alloc::vec::Vec;

use zune_jpeg::zune_core::colorspace::ColorSpace;
use zune_jpeg::zune_core::options::DecoderOptions;
use zune_jpeg::JpegDecoder;

pub struct FrameDecoder {
    rgb: Vec<u8>,
}

impl FrameDecoder {
    pub fn new() -> FrameDecoder {
        FrameDecoder { rgb: Vec::new() }
    }

    pub fn decode(
        &mut self,
        jpeg: &[u8],
        w: u32,
        h: u32,
        out: &mut [u32],
    ) -> Result<(), &'static str> {
        let pixels = (w as usize)
            .checked_mul(h as usize)
            .ok_or("frame dimensions overflow")?;
        let need = pixels.checked_mul(3).ok_or("frame dimensions overflow")?;
        if out.len() < pixels {
            return Err("frame buffer too small");
        }
        let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
        let mut d = JpegDecoder::new_with_options(jpeg, options);
        d.decode_headers().map_err(|_| "jpeg header invalid")?;
        if d.output_buffer_size() != Some(need) {
            return Err("frame dimensions differ from stream header");
        }
        if self.rgb.len() < need {
            self.rgb
                .try_reserve(need - self.rgb.len())
                .map_err(|_| "frame buffer allocation failed")?;
            self.rgb.resize(need, 0);
        }
        d.decode_into(&mut self.rgb[..need]).map_err(|_| "jpeg decode failed")?;
        for (px, rgb) in out[..pixels].iter_mut().zip(self.rgb[..need].chunks_exact(3)) {
            *px = 0xff00_0000 | ((rgb[0] as u32) << 16) | ((rgb[1] as u32) << 8) | rgb[2] as u32;
        }
        Ok(())
    }
}
