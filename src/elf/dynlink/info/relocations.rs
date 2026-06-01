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

use crate::elf::types::RelaEntry;

use super::state::DynLinkInfo;

impl DynLinkInfo {
    pub fn rela_count(&self) -> usize { self.rela_size / RelaEntry::SIZE }
    pub fn plt_rela_count(&self) -> usize { self.plt_rela_size / RelaEntry::SIZE }
    pub fn total_relocation_count(&self) -> usize { self.rela_count() + self.plt_rela_count() }
}
