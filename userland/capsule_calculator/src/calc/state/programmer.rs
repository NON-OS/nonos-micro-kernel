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

use super::State;
use crate::calc::fixed::{Fixed, FRAC};
use crate::calc::prog::mask;

const LOW: Fixed = i32::MIN as Fixed;
const HIGH: Fixed = i32::MAX as Fixed;

impl State {
    pub fn enter_programmer(&mut self) {
        let whole = (self.display / FRAC).clamp(LOW, HIGH);
        self.prog = mask(whole as i64);
        self.prog_acc = 0;
        self.prog_op = None;
        self.new_input = true;
    }
    pub fn leave_programmer(&mut self) {
        self.display = (self.prog as Fixed).saturating_mul(FRAC);
        self.operand = 0;
        self.new_input = true;
    }
}
