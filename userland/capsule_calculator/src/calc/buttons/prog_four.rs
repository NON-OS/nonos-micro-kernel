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
use crate::calc::prog::Bitwise;

pub const ROW: [Button; 6] = [
    b("4", Role::Number, Action::Digit(4)),
    b("5", Role::Number, Action::Digit(5)),
    b("6", Role::Number, Action::Digit(6)),
    b(">>", Role::Operator, Action::Bitwise(Bitwise::Shr)),
    b("XOR", Role::Operator, Action::Bitwise(Bitwise::Xor)),
    b("±", Role::Function, Action::Negate),
];
