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

use core::arch::asm;

use super::super::error::{PmpError, PmpResult};

pub fn write_cfg(index: usize, cfg: u8) -> PmpResult<()> {
    let shift = (index % 8) * 8;
    let mask = !(0xff_u64 << shift);
    let value = (cfg as u64) << shift;
    let group = index / 8;
    let current = read_word(group)?;
    write_word(group, (current & mask) | value)
}

fn read_word(group: usize) -> PmpResult<u64> {
    match group {
        0 => Ok(unsafe {
            let value: u64;
            asm!("csrr {}, pmpcfg0", out(reg) value);
            value
        }),
        1 => Ok(unsafe {
            let value: u64;
            asm!("csrr {}, pmpcfg2", out(reg) value);
            value
        }),
        _ => Err(PmpError::InvalidIndex),
    }
}

fn write_word(group: usize, value: u64) -> PmpResult<()> {
    match group {
        0 => unsafe { asm!("csrw pmpcfg0, {}", in(reg) value) },
        1 => unsafe { asm!("csrw pmpcfg2, {}", in(reg) value) },
        _ => return Err(PmpError::InvalidIndex),
    }
    Ok(())
}
