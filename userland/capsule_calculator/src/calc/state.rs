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
use super::hit::Hit;
use super::manifest::{HEIGHT, WIDTH};
use super::mode::Mode;
use super::op::Op;
use super::prog::{Base, Bitwise};

mod programmer;
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
    pub hover: Option<Hit>,
    pub view: (i32, i32),
    pub display: Fixed,
    pub operand: Fixed,
    pub operator: Op,
    pub memory: Fixed,
    pub new_input: bool,
    pub decimal_digits_typed: u8,
    pub error: ErrorKind,
    pub prog: i64,
    pub prog_acc: i64,
    pub prog_op: Option<Bitwise>,
    pub base: Base,
}

impl State {
    pub fn new() -> Self {
        State {
            mode: Mode::Basic,
            hover: None,
            view: (WIDTH as i32, HEIGHT as i32),
            display: 0,
            operand: 0,
            operator: Op::None,
            memory: 0,
            new_input: true,
            decimal_digits_typed: 0,
            error: ErrorKind::None,
            prog: 0,
            prog_acc: 0,
            prog_op: None,
            base: Base::Dec,
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
