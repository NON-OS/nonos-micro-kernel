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

// Minimal SVG rasterizer: paths with bezier and arc segments, the basic
// shapes, groups with affine transforms, solid fills (nonzero and evenodd)
// and approximated strokes, supersampled 2x. Gradients, masks, filters,
// text and use/defs indirection are skipped, never guessed.

mod affine;
mod arc;
mod attr;
mod color;
mod curves;
mod decode;
mod downsample;
mod draw;
mod fill;
mod math;
mod num;
mod path;
mod path_curves;
mod path_num;
mod path_state;
mod path_tok;
mod raster;
mod shapes;
mod state;
mod stroke;
mod transform;
mod walk;
mod xml;

pub(super) use decode::{decode_svg, is_svg};
