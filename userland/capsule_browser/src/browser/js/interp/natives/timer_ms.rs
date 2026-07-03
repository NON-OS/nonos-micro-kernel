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

use crate::browser::js::value::Value;

use super::super::to_num::to_num;

// Timer delay in ms, clamped to an hour so a bad page cannot park work
// unreachably far out.
pub(super) fn timer_ms(v: &Value) -> u32 {
    let n = to_num(v);
    if n.is_finite() && n > 0.0 {
        n.min(3_600_000.0) as u32
    } else {
        0
    }
}
