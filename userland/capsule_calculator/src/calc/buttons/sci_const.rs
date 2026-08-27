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

use super::kinds::{b, Action, Button, Role};
use crate::calc::op::Op;
use crate::calc::sci::Konst;

pub const ROW: [Button; 7] = [
    b("π", Role::Function, Action::Const(Konst::Pi)),
    b("e", Role::Function, Action::Const(Konst::E)),
    b("1/x", Role::Function, Action::Reciprocal),
    b("1", Role::Number, Action::Digit(1)),
    b("2", Role::Number, Action::Digit(2)),
    b("3", Role::Number, Action::Digit(3)),
    b("+", Role::Operator, Action::Operator(Op::Add)),
];
