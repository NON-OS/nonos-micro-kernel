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

use super::entries::Entry;
use super::info_row::KV_ADV;
use super::open_with_table::handlers_for;

// The panel's inner padding, the advance a section heading costs, the gap
// between sections, and the band a row of quick-action tiles occupies.
pub const PAD: u32 = 16;
pub const SEC_H: u32 = 22;
pub const SEC_GAP: u32 = 8;
pub const QUICK_H: u32 = 32;

// Em sizes: the panel title, the location line under it, and every section
// heading. `MIN_UI_PX` clamps all three, draw and measure alike.
pub const TITLE_PX: f32 = 18.0;
pub const SUB_PX: f32 = 14.0;
pub const SEC_PX: f32 = 14.0;

/// How tall `info_dir` or `info_file` will run for `entry`. Both stack rows of
/// the constant `KV_ADV`, and the handler count comes from a table rather than
/// from the store, so the height is known before a pixel is drawn -- which is
/// what lets the hit-test place the sections under it without a PaintBuffer.
pub fn body_h(entry: &Entry) -> u32 {
    if entry.is_dir {
        return KV_ADV * 3;
    }
    let rows = 3 + handlers_for(&entry.full_path).len().max(1) as u32;
    KV_ADV * rows + KV_ADV / 2
}
