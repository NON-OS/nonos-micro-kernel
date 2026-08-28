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
use crate::calc::prog::{Base, Bitwise};

pub const ROW: [Button; 6] = [
    b("HEX", Role::Function, Action::SetBase(Base::Hex)),
    b("DEC", Role::Function, Action::SetBase(Base::Dec)),
    b("OCT", Role::Function, Action::SetBase(Base::Oct)),
    b("BIN", Role::Function, Action::SetBase(Base::Bin)),
    b("NOT", Role::Operator, Action::Bitwise(Bitwise::Not)),
    b("AC", Role::Function, Action::Clear),
];
