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
    pub fn set_uid(&mut self, uid: u64) -> &mut Self {
        self.add(aux_type::AT_UID, uid)
    }
    pub fn set_euid(&mut self, euid: u64) -> &mut Self {
        self.add(aux_type::AT_EUID, euid)
    }
    pub fn set_gid(&mut self, gid: u64) -> &mut Self {
        self.add(aux_type::AT_GID, gid)
    }
    pub fn set_egid(&mut self, egid: u64) -> &mut Self {
        self.add(aux_type::AT_EGID, egid)
    }
    pub fn set_platform(&mut self, addr: VirtAddr) -> &mut Self {
        self.add(aux_type::AT_PLATFORM, addr.as_u64())
    }
    pub fn set_hwcap(&mut self, hwcap: u64) -> &mut Self {
        self.add(aux_type::AT_HWCAP, hwcap)
    }
    pub fn set_hwcap2(&mut self, hwcap2: u64) -> &mut Self {
        self.add(aux_type::AT_HWCAP2, hwcap2)
    }
    pub fn set_clktck(&mut self, ticks: u64) -> &mut Self {
        self.add(aux_type::AT_CLKTCK, ticks)
    }
    pub fn set_secure(&mut self, secure: bool) -> &mut Self {
        self.add(aux_type::AT_SECURE, if secure { 1 } else { 0 })
    }
    pub fn set_random(&mut self, addr: VirtAddr) -> &mut Self {
        self.add(aux_type::AT_RANDOM, addr.as_u64())
    }
    pub fn set_execfn(&mut self, addr: VirtAddr) -> &mut Self {
        self.add(aux_type::AT_EXECFN, addr.as_u64())
    }
    pub fn set_sysinfo_ehdr(&mut self, addr: VirtAddr) -> &mut Self {
        self.add(aux_type::AT_SYSINFO_EHDR, addr.as_u64())
    }
}
