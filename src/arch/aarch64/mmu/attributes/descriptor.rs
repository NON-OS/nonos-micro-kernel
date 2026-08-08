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

use super::page::PageAttributes;
use super::pte::{
    PTE_AF, PTE_AP_RO_ALL, PTE_AP_RO_EL1, PTE_AP_RW_ALL, PTE_AP_RW_EL1, PTE_CONT, PTE_NG, PTE_PXN,
    PTE_SH_IS, PTE_UXN,
};

impl PageAttributes {
    /// The attribute bits of a block or page descriptor. The valid bit, the
    /// table/page bit and the output address belong to whoever writes the
    /// entry; this is only the permissions and the memory type.
    pub fn to_descriptor_bits(&self) -> u64 {
        let mut bits = self.memory_type.attr_index() << 2;

        // AP[2:1] says both who may reach the page and whether they may write
        // it. EL0 reaches a page only when AP[1] is set, so that bit tracks
        // `user` directly; inverting it would publish the whole kernel to
        // userspace and lock userspace out of its own pages.
        bits |= match (self.user, self.write) {
            (false, true) => PTE_AP_RW_EL1,
            (false, false) => PTE_AP_RO_EL1,
            (true, true) => PTE_AP_RW_ALL,
            (true, false) => PTE_AP_RO_ALL,
        };

        // Inner shareable. Ignored for Device memory, and right for every
        // Normal mapping this kernel makes.
        bits |= PTE_SH_IS;

        if self.accessed {
            bits |= PTE_AF;
        }
        if !self.global {
            bits |= PTE_NG;
        }
        if self.contiguous {
            bits |= PTE_CONT;
        }

        // Execute permission never crosses the privilege boundary, whatever
        // `execute` asks for: EL1 must not run user pages and EL0 must not run
        // kernel pages. Within a page's own level, `execute` decides.
        if self.user || !self.execute {
            bits |= PTE_PXN;
        }
        if !self.user || !self.execute {
            bits |= PTE_UXN;
        }

        bits
    }
}
