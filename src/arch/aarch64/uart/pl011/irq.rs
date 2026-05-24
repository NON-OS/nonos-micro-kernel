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

use super::constants::{IMSC_RXIM, INTERRUPT_CLEAR_ALL, UARTICR, UARTIMSC, UARTMIS};
use super::device::Pl011;

impl Pl011 {
    pub fn enable_rx_interrupt(&self) {
        let imsc = self.read_reg(UARTIMSC);
        self.write_reg(UARTIMSC, imsc | IMSC_RXIM);
    }

    pub fn disable_rx_interrupt(&self) {
        let imsc = self.read_reg(UARTIMSC);
        self.write_reg(UARTIMSC, imsc & !IMSC_RXIM);
    }

    pub fn clear_interrupts(&self) {
        self.write_reg(UARTICR, INTERRUPT_CLEAR_ALL);
    }

    pub fn pending_interrupts(&self) -> u32 {
        self.read_reg(UARTMIS)
    }
}
