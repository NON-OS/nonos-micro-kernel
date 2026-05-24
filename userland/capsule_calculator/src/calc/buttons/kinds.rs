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

use crate::calc::op::Op;

#[derive(Clone, Copy)]
pub enum Role {
    Number,
    Operator,
    Equals,
    Function,
    Memory,
}

#[derive(Clone, Copy)]
pub enum Action {
    Digit(u8),
    Decimal,
    Operator(Op),
    Equals,
    Clear,
    Negate,
    Percent,
    Square,
    SquareRoot,
    Reciprocal,
    MemoryAdd,
    MemorySub,
    MemoryRecall,
    MemoryClear,
    MemoryStore,
}

#[derive(Clone, Copy)]
pub struct Button {
    pub label: &'static [u8],
    pub role: Role,
    pub action: Action,
}

pub const fn b(label: &'static [u8], role: Role, action: Action) -> Button {
    Button { label, role, action }
}
