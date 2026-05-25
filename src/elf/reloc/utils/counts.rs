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

use super::supported::is_supported;
use crate::elf::types::RelaEntry;

pub fn count_supported(rela_entries: &[RelaEntry]) -> usize {
    rela_entries.iter().filter(|entry| is_supported(entry.reloc_type())).count()
}

pub fn count_unsupported(rela_entries: &[RelaEntry]) -> usize {
    rela_entries.iter().filter(|entry| !is_supported(entry.reloc_type())).count()
}
