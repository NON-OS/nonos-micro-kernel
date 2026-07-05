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

use super::{color_tf, green_tf, indexing, predictor};

// A decoded transform holding whatever its inverse needs. Predictor and color
// keep their block data and the block width they were sampled at; indexing
// keeps the palette and the full width to expand back to.
pub(super) enum Transform {
    Predictor { data: Vec<u32>, bits: u32, dw: usize },
    Color { data: Vec<u32>, bits: u32, dw: usize },
    SubtractGreen,
    ColorIndex { pal: Vec<u32>, orig_w: usize },
}

// Apply the transforms in reverse of read order. Color indexing expands the
// bundled image back to full width, so it returns the possibly grown width.
pub(super) fn apply_all(
    transforms: Vec<Transform>,
    mut px: Vec<u32>,
    mut w: usize,
    h: usize,
) -> (Vec<u32>, usize) {
    for t in transforms.into_iter().rev() {
        match t {
            Transform::Predictor { data, bits, dw } => {
                predictor::apply(&mut px, w, h, &data, bits, dw)
            }
            Transform::Color { data, bits, dw } => color_tf::apply(&mut px, w, h, &data, bits, dw),
            Transform::SubtractGreen => green_tf::apply(&mut px),
            Transform::ColorIndex { pal, orig_w } => {
                px = indexing::apply(&px, w, h, orig_w, &pal);
                w = orig_w;
            }
        }
    }
    (px, w)
}
