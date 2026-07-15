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

use super::super::error::{IoApicError, IoApicResult};
use super::super::mmio::{redtbl_read, redtbl_write};
use super::super::ops_helpers::locate;

pub fn mask(gsi: u32, masked: bool) -> IoApicResult<()> {
    let (chip, idx) = locate(gsi).ok_or(IoApicError::GsiNotFound)?;
    unsafe {
        let (mut low, high) = redtbl_read(chip.mmio, idx);
        if masked {
            low |= 1 << 16;
        } else {
            low &= !(1 << 16);
        }
        redtbl_write(chip.mmio, idx, low, high);
    }
    Ok(())
}
