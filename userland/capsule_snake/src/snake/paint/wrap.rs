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

use crate::snake::ui::text;

// The longest word-boundary prefix that measures within `max_w`, plus what is
// left. Nothing here counts glyphs: the cut comes from the measured fit and is
// only walked back to a space, which the body face has no say in.
pub fn split(bytes: &[u8], px: f32, max_w: u32) -> (&[u8], &[u8]) {
    let cut = text::fit(bytes, px, max_w);
    if cut.len() == bytes.len() {
        return (bytes, &bytes[..0]);
    }
    let mut end = cut.len();
    while end > 0 && bytes[end] != b' ' {
        end -= 1;
    }
    if end == 0 {
        return (cut, &bytes[cut.len()..]);
    }
    (&bytes[..end], &bytes[end + 1..])
}
