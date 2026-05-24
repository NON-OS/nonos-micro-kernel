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
use super::device::Pl011;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pl011ConfigError {
    InvalidBaud,
    InvalidClock,
}

pub type Pl011ConfigResult<T> = Result<T, Pl011ConfigError>;

impl Pl011 {
    pub fn init(&self, baud: u32, clock: u32) -> Pl011ConfigResult<()> {
        let denom = 16u64
            .checked_mul(baud as u64)
            .ok_or(Pl011ConfigError::InvalidBaud)?;
        if denom == 0 || clock as u64 < denom {
            return Err(Pl011ConfigError::InvalidClock);
        }
        let mut integer = clock as u64 / denom;
        let rem = clock as u64 % denom;
        let mut fractional = ((rem * 64) + (denom / 2)) / denom;
        if fractional == 64 {
            integer += 1;
            fractional = 0;
        }
        self.write_reg(UARTCR, 0);
        self.write_reg(UARTIBRD, integer as u32);
        self.write_reg(UARTFBRD, fractional as u32);
        self.write_reg(UARTLCR_H, LCR_WLEN_8 | LCR_FEN);
        self.write_reg(UARTCR, CR_UARTEN | CR_TXE | CR_RXE);
        Ok(())
    }
}
