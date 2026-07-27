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

pub const PERIOD_BYTES: u64 = 0x2000;
pub const N_PERIODS: usize = 4;
pub const RING_BYTES: u64 = PERIOD_BYTES * N_PERIODS as u64;
pub const BDL_IOC: u32 = 1;

pub struct BdlEntry {
    pub addr: u64,
    pub len: u32,
    pub flags: u32,
}

pub fn build_bdl(ring_base: u64) -> [BdlEntry; N_PERIODS] {
    core::array::from_fn(|i| BdlEntry {
        addr: ring_base + i as u64 * PERIOD_BYTES,
        len: PERIOD_BYTES as u32,
        flags: BDL_IOC,
    })
}
