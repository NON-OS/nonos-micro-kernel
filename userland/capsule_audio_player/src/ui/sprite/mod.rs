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

//! Anti-aliased RGBA8 glyph rasteriser. Every mask is built once at start-up
//! by `icon::Icons` and tinted at blit time, so no frame pays for supersampling.

mod canvas;
mod glyph_a;
mod glyph_b;
mod glyph_c;
mod prim;
mod shape;
mod stroke;
mod transport_a;
mod transport_b;

pub use canvas::Sprite;
pub use glyph_a::{magnifier, note, speaker};
pub use glyph_b::{check, close, plus};
pub use glyph_c::{bell, chevron, compass, download, gear, grid, heart, home, radio};
pub use transport_a::{next, pause, play, prev};
pub use transport_b::{repeat, shuffle};
