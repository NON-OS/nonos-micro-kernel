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

use super::constants::{LSR, LSR_DR, LSR_THRE, RBR};
use super::device::Ns16550;

impl Ns16550 {
    pub fn getc(&self) -> Option<u8> {
        if self.is_rx_ready() {
            Some(self.read_reg(RBR))
        } else {
            None
        }
    }

    pub fn getc_blocking(&self) -> u8 {
        while !self.is_rx_ready() {
            core::hint::spin_loop();
        }
        self.read_reg(RBR)
    }

    pub fn is_rx_ready(&self) -> bool {
        self.read_reg(LSR) & LSR_DR != 0
    }

    pub fn is_tx_ready(&self) -> bool {
        self.read_reg(LSR) & LSR_THRE != 0
    }
}
