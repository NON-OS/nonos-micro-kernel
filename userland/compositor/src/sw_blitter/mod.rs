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

pub mod copy_rect;
pub mod fill;
pub mod row;

pub use copy_rect::composite_layer;
pub use fill::fill_rect;

#[derive(Clone, Copy, Default)]
pub struct Surface {
    pub base_va: u64,
    pub stride: u32,
    pub width: u32,
    pub height: u32,
    pub byte_len: u64,
}

impl Surface {
    pub fn row_start(self, y: u32, x: u32, width: u32) -> Option<usize> {
        if width == 0 || y >= self.height || x >= self.width || width > self.width - x {
            return None;
        }
        let row_off = (y as u64).checked_mul(self.stride as u64)?;
        let col_off = (x as u64).checked_mul(4)?;
        let span_bytes = (width as u64).checked_mul(4)?;
        let start = row_off.checked_add(col_off)?;
        let end = start.checked_add(span_bytes)?;
        if end > self.byte_len {
            return None;
        }
        usize::try_from(self.base_va.checked_add(start)?).ok()
    }
}
