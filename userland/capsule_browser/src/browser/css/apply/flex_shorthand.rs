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

use crate::browser::css::computed::{Computed, Size};
use crate::browser::css::parse_grow::parse_grow;
use crate::browser::css::parse_size::parse_size;

// flex shorthand: none, auto, or "<grow> [<shrink>] [<basis>]". The single
// number form "flex: 1" means grow 1 with a zero basis, so items share the
// row equally rather than by content width. none is 0 0 auto; auto is 1 1
// auto.
pub(super) fn apply_flex_shorthand(c: &mut Computed, v: &str, fs: u32) {
    match v {
        "none" => {
            c.flex_grow = 0;
            c.flex_basis = Size::Auto;
            return;
        }
        "auto" => {
            c.flex_grow = 1;
            c.flex_basis = Size::Auto;
            return;
        }
        _ => {}
    }
    let mut basis_set = false;
    let mut nums = 0u8;
    for tok in v.split_whitespace() {
        if let Some(s) = parse_size(tok, fs).filter(|_| tok.contains(|ch: char| !ch.is_numeric())) {
            c.flex_basis = s;
            basis_set = true;
        } else if let Some(g) = parse_grow(tok) {
            if nums == 0 {
                c.flex_grow = g;
            }
            nums += 1;
        }
    }
    // A unitless single number gives basis 0, the equal-share default.
    if !basis_set {
        c.flex_basis = Size::Px(0);
    }
}
