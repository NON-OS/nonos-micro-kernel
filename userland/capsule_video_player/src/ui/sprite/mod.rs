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

//! Anti-aliased RGBA8 glyph rasteriser shared by every video-player icon.

mod arrow;
mod canvas;
mod downscale;
mod kind;
mod mark;
mod media;
mod mode;
mod nav;
mod prim;
mod shape;
mod stroke;
mod table;
mod tool;
mod transport;
mod unit;
mod view;

pub mod cache;

pub use kind::Glyph;
