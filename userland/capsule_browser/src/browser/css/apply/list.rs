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

// list-style-type, and the position in the list-style shorthand where the
// type keyword may appear. Only none-vs-marker matters to the renderer.
pub(super) fn apply_list(c: &mut Computed, name: &str, value: &str) -> bool {
    match name {
        "list-style" | "list-style-type" => {
            let v = value.to_ascii_lowercase();
            if v.split_whitespace().any(|t| t == "none") {
                c.list_none = true;
            } else if !v.trim().is_empty() {
                c.list_none = false;
            }
        }
        _ => return false,
    }
    true
}
