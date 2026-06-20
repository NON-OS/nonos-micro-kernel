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

use crate::term::grid::types::Grid;

pub fn decset(g: &mut Grid, params: &[i64], inter: &[u8], set: bool) {
    if !inter.contains(&b'?') {
        return;
    }
    match params.first().copied().unwrap_or(0) {
        25 => g.cursor_visible = set,
        47 | 1047 => {
            if set {
                g.enter_alt(false);
            } else {
                g.leave_alt();
            }
        }
        1049 => {
            if set {
                g.enter_alt(true);
            } else {
                g.leave_alt();
            }
        }
        _ => {}
    }
}
