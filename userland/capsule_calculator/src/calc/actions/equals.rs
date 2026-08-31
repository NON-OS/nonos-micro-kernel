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

use crate::calc::history::{write_expr, EXPR_MAX};
use crate::calc::op::{apply, Op};
use crate::calc::state::State;

pub fn run(state: &mut State) {
    if state.is_error() || state.operator == Op::None {
        return;
    }
    let (lhs, rhs, op) = (state.operand, state.display, state.operator);
    match apply(lhs, rhs, op) {
        Ok(result) => {
            state.display = result;
            let mut buf = [0u8; EXPR_MAX];
            let n = write_expr(lhs, op, rhs, &mut buf);
            state.history.push(&buf[..n], result);
        }
        Err(kind) => {
            state.error = kind;
            state.display = 0;
        }
    }
    state.operator = Op::None;
    state.operand = 0;
    state.reset_input();
}
