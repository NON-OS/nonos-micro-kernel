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

use super::{prog_bitwise, prog_digit};
use crate::calc::buttons::Action;
use crate::calc::prog::mask;
use crate::calc::state::State;

pub fn run(state: &mut State, action: Action) -> bool {
    match action {
        Action::Digit(d) => prog_digit::run(state, d),
        Action::Bitwise(op) => prog_bitwise::run(state, op),
        Action::Equals => prog_bitwise::resolve(state),
        Action::SetBase(base) => {
            state.base = base;
            state.new_input = true;
        }
        Action::Clear => {
            state.prog = 0;
            state.prog_acc = 0;
            state.prog_op = None;
            state.new_input = true;
        }
        Action::Negate => {
            state.prog = mask(state.prog.wrapping_neg());
            state.new_input = true;
        }
        _ => {}
    }
    true
}
