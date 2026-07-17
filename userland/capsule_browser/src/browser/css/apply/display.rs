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

use crate::browser::css::computed::Computed;

// The display property routes a box to its formatting context.
pub(super) fn apply_display(c: &mut Computed, name: &str, value: &str) -> bool {
    if name != "display" {
        return false;
    }
    match value.trim() {
        "none" => c.display_none = true,
        "block" | "list-item" => {
            c.is_block = true;
            c.is_flex = false;
            c.is_grid = false;
        }
        "table" | "inline-table" => {
            c.is_block = true;
            c.is_flex = false;
            c.is_grid = false;
            c.is_table = true;
        }
        "table-row" | "table-row-group" | "table-header-group" | "table-footer-group" => {
            c.is_block = true;
            c.is_table_row = value.trim() == "table-row";
        }
        "table-cell" => {
            c.is_block = true;
            c.is_table_cell = true;
        }
        "flex" => {
            c.is_block = true;
            c.is_flex = true;
            c.is_grid = false;
        }
        "inline-flex" => {
            c.is_block = false;
            c.is_flex = true;
            c.is_grid = false;
        }
        "grid" => {
            c.is_block = true;
            c.is_flex = false;
            c.is_grid = true;
        }
        "inline-grid" => {
            c.is_block = false;
            c.is_flex = false;
            c.is_grid = true;
        }
        "inline" => {
            c.is_block = false;
            c.is_flex = false;
            c.is_grid = false;
            c.is_inline_block = false;
        }
        "inline-block" => {
            c.is_block = false;
            c.is_flex = false;
            c.is_grid = false;
            c.is_inline_block = true;
        }
        _ => {}
    }
    true
}
