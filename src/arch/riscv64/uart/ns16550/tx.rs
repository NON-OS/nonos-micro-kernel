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

use super::constants::{LSR, LSR_TEMT, LSR_THRE, THR};
use super::device::Ns16550;

impl Ns16550 {
    pub fn putc(&self, c: u8) {
        while self.read_reg(LSR) & LSR_THRE == 0 {
            core::hint::spin_loop();
        }
        self.write_reg(THR, c);
    }

    pub fn puts(&self, s: &[u8]) {
        for &c in s {
            if c == b'\n' {
                self.putc(b'\r');
            }
            self.putc(c);
        }
    }

    pub fn flush(&self) {
        while self.read_reg(LSR) & LSR_TEMT == 0 {
            core::hint::spin_loop();
        }
    }
}
