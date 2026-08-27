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

use crate::calc::state::State;
use crate::calc::ui::convert_hit::{at, ConvertHit};

pub fn click(state: &mut State, x: i32, y: i32) -> bool {
    match at(state.cat, state.view.0, x, y) {
        Some(ConvertHit::Chip(i)) => state.set_category(i),
        Some(ConvertHit::From(i)) => state.set_unit(true, i),
        Some(ConvertHit::To(i)) => state.set_unit(false, i),
        Some(ConvertHit::Swap) => state.swap_units(),
        None => return false,
    }
    true
}
