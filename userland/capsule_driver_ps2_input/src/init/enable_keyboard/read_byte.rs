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
use crate::constants::{DATA_OFFSET, STATUS_OFFSET, STATUS_OUTPUT_FULL};
use nonos_libc::mk_pio_read;

const WAIT_SPINS: u32 = 100_000;

pub(super) fn read_byte(grant_id: u64) -> Result<u8, &'static str> {
    for _ in 0..WAIT_SPINS {
        let mut status = 0u32;
        if mk_pio_read(grant_id, STATUS_OFFSET, 1, &mut status) < 0 {
            return Err("kbd status read failed");
        }
        if status as u8 & STATUS_OUTPUT_FULL != 0 {
            let mut value = 0u32;
            if mk_pio_read(grant_id, DATA_OFFSET, 1, &mut value) < 0 {
                return Err("kbd data read failed");
            }
            return Ok(value as u8);
        }
    }
    Err("kbd read timeout")
}
