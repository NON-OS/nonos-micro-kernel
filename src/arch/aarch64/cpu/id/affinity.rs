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

use super::mpidr::mpidr;

pub fn cpu_id() -> usize {
    let value = mpidr();
    let aff0 = (value & 0xFF) as usize;
    let aff1 = ((value >> 8) & 0xFF) as usize;
    let aff2 = ((value >> 16) & 0xFF) as usize;
    (aff2 << 8) | (aff1 << 4) | aff0
}

pub fn core_id() -> usize {
    (mpidr() & 0xFF) as usize
}

pub fn cluster_id() -> usize {
    ((mpidr() >> 8) & 0xFF) as usize
}

pub fn affinity_level(level: u32) -> u64 {
    let value = mpidr();
    match level {
        0 => value & 0xFF,
        1 => (value >> 8) & 0xFF,
        2 => (value >> 16) & 0xFF,
        3 => (value >> 32) & 0xFF,
        _ => 0,
    }
}

pub fn is_primary_core() -> bool {
    cpu_id() == 0
}

pub fn is_multiprocessor() -> bool {
    (mpidr() & (1 << 30)) == 0
}
