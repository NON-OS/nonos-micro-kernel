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

pub const ROW: [Button; 5] = [
    b("7", Role::Number, Action::Digit(7)),
    b("8", Role::Number, Action::Digit(8)),
    b("9", Role::Number, Action::Digit(9)),
    b("x^2", Role::Function, Action::Square),
    b("*", Role::Operator, Action::Operator(Op::Mul)),
];
