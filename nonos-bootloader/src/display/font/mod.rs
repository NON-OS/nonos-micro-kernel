// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

pub mod bitmap;
mod draw;
mod metrics;
mod scale;

pub use bitmap::get_char_bitmap;
pub use draw::{
    draw_char, draw_char_2x, draw_hash_bytes, draw_hex_byte, draw_string, draw_string_2x,
};
pub use metrics::{CHAR_HEIGHT, CHAR_WIDTH};
pub use scale::{draw_char_3x, draw_char_4x, draw_string_3x, draw_string_4x};
