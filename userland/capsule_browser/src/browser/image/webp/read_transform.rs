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

use super::bitread::BitReader;
use super::decode_image::decode_image;
use super::indexing::{bundle_params, delta_decode};
use super::meta::subsample_size;
use super::transform::Transform;

// Read one transform. Predictor and color carry a block sub-image sampled at
// the current width. Color indexing carries a palette and, for a small one,
// shrinks the width the main image is stored bundled at.
pub(super) fn read_transform(
    br: &mut BitReader,
    xsize: &mut usize,
    ysize: usize,
) -> Option<Transform> {
    match br.read(2) {
        0 => {
            let (data, bits, dw) = read_block(br, *xsize, ysize)?;
            Some(Transform::Predictor { data, bits, dw })
        }
        1 => {
            let (data, bits, dw) = read_block(br, *xsize, ysize)?;
            Some(Transform::Color { data, bits, dw })
        }
        2 => Some(Transform::SubtractGreen),
        _ => {
            let num = br.read(8) as usize + 1;
            let mut pal = decode_image(br, num, 1, false)?;
            delta_decode(&mut pal);
            let orig_w = *xsize;
            let (_, wbits) = bundle_params(num);
            if wbits > 0 {
                *xsize = subsample_size(*xsize, wbits);
            }
            Some(Transform::ColorIndex { pal, orig_w })
        }
    }
}

// The per-block data image for predictor and color transforms.
fn read_block(
    br: &mut BitReader,
    xsize: usize,
    ysize: usize,
) -> Option<(alloc::vec::Vec<u32>, u32, usize)> {
    let bits = br.read(3) + 2;
    let dw = subsample_size(xsize, bits);
    let dh = subsample_size(ysize, bits);
    let data = decode_image(br, dw, dh, false)?;
    Some((data, bits, dw))
}
