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

use super::super::types::aux_type;
use super::state::AuxvBuilder;
use crate::memory::addr::VirtAddr;

impl AuxvBuilder {
    pub fn set_phdr(&mut self, addr: VirtAddr) -> &mut Self {
        self.add(aux_type::AT_PHDR, addr.as_u64())
    }
    pub fn set_phent(&mut self, size: u64) -> &mut Self {
        self.add(aux_type::AT_PHENT, size)
    }
    pub fn set_phnum(&mut self, count: u64) -> &mut Self {
        self.add(aux_type::AT_PHNUM, count)
    }
    pub fn set_pagesz(&mut self, size: u64) -> &mut Self {
        self.add(aux_type::AT_PAGESZ, size)
    }
    pub fn set_base(&mut self, addr: VirtAddr) -> &mut Self {
        self.add(aux_type::AT_BASE, addr.as_u64())
    }
    pub fn set_flags(&mut self, flags: u64) -> &mut Self {
        self.add(aux_type::AT_FLAGS, flags)
    }
    pub fn set_entry(&mut self, addr: VirtAddr) -> &mut Self {
        self.add(aux_type::AT_ENTRY, addr.as_u64())
    }
}
