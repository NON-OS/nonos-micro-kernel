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

use super::fixed::Fixed;
use super::mode::Mode;
use super::op::Op;

mod set_mode;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    None,
    DivByZero,
    DomainError,
    Overflow,
}

pub struct State {
    pub mode: Mode,
    pub hover: Option<Mode>,
    pub display: Fixed,
    pub operand: Fixed,
    pub operator: Op,
    pub memory: Fixed,
    pub new_input: bool,
    pub decimal_digits_typed: u8,
    pub error: ErrorKind,
}

impl State {
    pub fn new() -> Self {
        State {
            mode: Mode::Basic,
            hover: None,
            display: 0,
            operand: 0,
            operator: Op::None,
            memory: 0,
            new_input: true,
            decimal_digits_typed: 0,
            error: ErrorKind::None,
        }
    }
    pub fn memory_engaged(&self) -> bool {
        self.memory != 0
    }
    pub fn is_error(&self) -> bool {
        self.error != ErrorKind::None
    }
    pub fn reset_input(&mut self) {
        self.new_input = true;
        self.decimal_digits_typed = 0;
    }
}
