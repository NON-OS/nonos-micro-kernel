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

pub const ROW: [Button; 6] = [
    b("A", Role::Number, Action::Digit(10)),
    b("B", Role::Number, Action::Digit(11)),
    b("C", Role::Number, Action::Digit(12)),
    b("D", Role::Number, Action::Digit(13)),
    b("E", Role::Number, Action::Digit(14)),
    b("F", Role::Number, Action::Digit(15)),
];
