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

use crate::calc::prog::{mask, Bitwise};
use crate::calc::state::State;

pub fn resolve(state: &mut State) {
    let op = match state.prog_op.take() {
        Some(op) => op,
        None => return,
    };
    let (a, b) = (state.prog_acc, state.prog);
    state.prog = mask(match op {
        Bitwise::And => a & b,
        Bitwise::Or => a | b,
        Bitwise::Xor => a ^ b,
        _ => b,
    });
    state.new_input = true;
}

pub fn run(state: &mut State, op: Bitwise) {
    match op {
        Bitwise::Not => state.prog = mask(!state.prog),
        Bitwise::Shl => state.prog = mask(state.prog.wrapping_shl(1)),
        Bitwise::Shr => state.prog = mask(((state.prog as u32) >> 1) as i64),
        _ => {
            resolve(state);
            state.prog_acc = state.prog;
            state.prog_op = Some(op);
        }
    }
    state.new_input = true;
}
