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

use super::constants::*;
use super::device::Ns16550;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UartConfigError {
    InvalidBaud,
    InvalidClock,
}

pub type UartConfigResult<T> = Result<T, UartConfigError>;

impl Ns16550 {
    pub fn init(&self, baud: u32, clock: u32) -> UartConfigResult<()> {
        let divisor_base = 16u32.checked_mul(baud).ok_or(UartConfigError::InvalidBaud)?;
        if divisor_base == 0 {
            return Err(UartConfigError::InvalidBaud);
        }
        if clock < divisor_base {
            return Err(UartConfigError::InvalidClock);
        }
        let divisor = clock / divisor_base;
        self.write_reg(IER, 0);
        self.write_reg(LCR, LCR_DLAB);
        self.write_reg(DLL, (divisor & 0xFF) as u8);
        self.write_reg(DLM, ((divisor >> 8) & 0xFF) as u8);
        self.write_reg(LCR, LCR_8N1);
        self.write_reg(FCR, FCR_ENABLE | FCR_CLEAR_RX | FCR_CLEAR_TX | FCR_TRIGGER_14);
        self.write_reg(MCR, MCR_DTR | MCR_RTS | MCR_OUT2);
        self.write_reg(IER, IER_RDA);
        Ok(())
    }
}
