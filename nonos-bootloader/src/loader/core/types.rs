// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use goblin::elf::Elf;

use super::constants::MAX_LOADS;

pub struct ValidatedSegment {
    pub p_offset: usize,
    pub p_filesz: usize,
    pub p_memsz: usize,
    pub target: u64,
    pub p_align: usize,
    pub p_flags: u32,
}

pub struct ValidationResult<'a> {
    pub elf: Elf<'a>,
    pub loads: [ValidatedSegment; MAX_LOADS],
    pub load_count: usize,
    pub min_addr: u64,
    pub max_addr: u64,
    pub is_exec: bool,
    pub is_dyn: bool,
}

impl Default for ValidatedSegment {
    fn default() -> Self {
        Self { p_offset: 0, p_filesz: 0, p_memsz: 0, target: 0, p_align: 0, p_flags: 0 }
    }
}
