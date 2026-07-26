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

//! Anti-aliased RGBA8 sprite rasterizer for transport icons and art.

mod canvas;
mod fx;
mod glyph_a;
mod glyph_b;
mod hero;
mod prim;
mod shape;
mod stroke;
mod transport_a;
mod transport_b;

pub use canvas::Sprite;
pub use glyph_a::{magnifier, note, speaker};
pub use glyph_b::{check, close, plus};
pub use hero::{glow_disc, gradient_art};
pub use transport_a::{pause, play, prev, next, stop};
pub use transport_b::{repeat, shuffle};
