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

use crate::memory::addr::PhysAddr;
use crate::smp::constants::AP_TRAMPOLINE_ADDR;
use crate::smp::trampoline::install_trampoline_at;

pub(super) struct ApBootInputs {
    pub(super) pml4_phys: u64,
    pub(super) entry_ptr: u64,
}

pub(super) fn prepare() -> Result<ApBootInputs, &'static str> {
    install_trampoline_at(PhysAddr::new(AP_TRAMPOLINE_ADDR))
        .map_err(|_| "Failed to install AP trampoline")?;

    Ok(ApBootInputs {
        pml4_phys: read_cr3_pml4(),
        entry_ptr: crate::smp::ap_entry as *const () as usize as u64,
    })
}

fn read_cr3_pml4() -> u64 {
    crate::arch::paging::read_root()
}
