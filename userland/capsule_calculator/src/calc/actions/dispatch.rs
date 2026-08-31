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

use super::{
    clear, constant, decimal, digit, equals, memory_add, memory_clear, memory_recall, memory_store,
    memory_sub, negate, percent, prog_dispatch, reciprocal, sci_fn, set_op, square, square_root,
};
use crate::calc::buttons::Action;
use crate::calc::mode::Mode;
use crate::calc::state::State;

pub fn run(state: &mut State, action: Action) {
    if state.mode == Mode::Programmer && prog_dispatch::run(state, action) {
        return;
    }
    match action {
        Action::Digit(d) => digit::run(state, d),
        Action::Decimal => decimal::run(state),
        Action::Operator(op) => set_op::run(state, op),
        Action::Equals => equals::run(state),
        Action::Clear => clear::run(state),
        Action::Negate => negate::run(state),
        Action::Percent => percent::run(state),
        Action::Square => square::run(state),
        Action::SquareRoot => square_root::run(state),
        Action::Reciprocal => reciprocal::run(state),
        Action::MemoryAdd => memory_add::run(state),
        Action::MemorySub => memory_sub::run(state),
        Action::MemoryRecall => memory_recall::run(state),
        Action::MemoryClear => memory_clear::run(state),
        Action::MemoryStore => memory_store::run(state),
        Action::Sci(f) => sci_fn::run(state, f),
        Action::Const(k) => constant::run(state, k),
        Action::SetBase(_) | Action::Bitwise(_) => {}
    }
}
