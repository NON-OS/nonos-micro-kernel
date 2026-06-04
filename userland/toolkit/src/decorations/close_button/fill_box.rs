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
use super::types::CloseRect;

pub fn fill_box(pixels: &mut [u32], stride_words: usize, width: u32, rect: &CloseRect, argb: u32) {
    for row in rect.y..rect.y + rect.size {
        let base = (row as usize) * stride_words;
        for col in rect.x..rect.x + rect.size {
            let idx = base + col as usize;
            if col < width && idx < pixels.len() {
                pixels[idx] = argb;
            }
        }
    }
}
