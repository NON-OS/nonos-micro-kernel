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

// Antialiased vector text from a bundled TrueType face. Glyphs are
// outlined by ab_glyph and blended onto an ARGB8888 surface at CPL=3,
// where the FPU is live. Metrics come straight from the face, so
// layout and paint agree on advance widths.

mod blend;
mod cache;
mod draw;
mod face;
mod metrics;
mod readable;

pub use face::builtin_face;
pub use readable::MIN_UI_PX;

pub use ab_glyph::FontRef;
pub use draw::{draw_text, draw_text_spaced, draw_text_tracked, draw_text_with};
pub use metrics::{
    ascent, ascent_with, line_height, line_height_with, measure, measure_spaced, measure_tracked,
    measure_with,
};
