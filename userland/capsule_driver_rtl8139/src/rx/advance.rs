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

use crate::constants::dma::RX_BUF_DATA_BYTES;
use crate::constants::regs::REG_CAPR;
use crate::setup::Driver;

pub(super) fn advance(driver: &mut Driver, raw_len: usize) -> Result<(), &'static str> {
    let next = (driver.rx_offset + raw_len + 4 + 3) & !3;
    driver.rx_offset = next % RX_BUF_DATA_BYTES;
    let capr = (driver.rx_offset + RX_BUF_DATA_BYTES - 16) % RX_BUF_DATA_BYTES;
    driver.pio.w16(REG_CAPR, capr as u16)
}
