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

use crate::browser::css::computed::{Align, Computed, Justify};

// Main-axis justification and cross-axis item alignment.
pub(super) fn apply_align(c: &mut Computed, name: &str, value: &str) -> bool {
    match name {
        "justify-content" => match value.trim() {
            "flex-start" | "start" | "left" => c.justify = Justify::Start,
            "center" => c.justify = Justify::Center,
            "flex-end" | "end" | "right" => c.justify = Justify::End,
            "space-between" => c.justify = Justify::Between,
            "space-around" | "space-evenly" => c.justify = Justify::Around,
            _ => {}
        },
        "align-items" => match value.trim() {
            "flex-start" | "start" | "baseline" => c.align = Align::Start,
            "center" => c.align = Align::Center,
            "flex-end" | "end" => c.align = Align::End,
            "stretch" => c.align = Align::Stretch,
            _ => {}
        },
        _ => return false,
    }
    true
}
