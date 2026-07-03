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
use crate::browser::css::parse_grow::parse_grow;
use crate::browser::css::parse_size::parse_size;

// Flex container axis and wrapping, and the item grow factor.
pub(super) fn apply_flex(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        "flex-direction" => {
            let v = value.trim();
            if v.starts_with("column") {
                c.flex_col = true;
            } else if v.starts_with("row") {
                c.flex_col = false;
            }
        }
        // Shorthand for flex-direction and flex-wrap in either order. Both
        // longhands reset first: an omitted part takes its initial value.
        "flex-flow" => {
            c.flex_col = false;
            c.flex_wrap = false;
            for tok in value.split_whitespace() {
                if tok.starts_with("column") {
                    c.flex_col = true;
                } else if tok.starts_with("row") {
                    c.flex_col = false;
                } else if tok == "wrap" || tok == "wrap-reverse" {
                    c.flex_wrap = true;
                }
            }
        }
        "flex-wrap" => match value.trim() {
            "wrap" | "wrap-reverse" => c.flex_wrap = true,
            "nowrap" => c.flex_wrap = false,
            _ => {}
        },
        "flex-grow" => {
            if let Some(g) = parse_grow(value) {
                c.flex_grow = g;
            }
        }
        "flex-basis" => {
            if let Some(s) = parse_size(value, fs) {
                c.flex_basis = s;
            }
        }
        "flex" => super::flex_shorthand::apply_flex_shorthand(c, value.trim(), fs),
        _ => return false,
    }
    true
}
