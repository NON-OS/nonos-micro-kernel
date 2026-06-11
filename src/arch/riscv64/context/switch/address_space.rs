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

use alloc::sync::Arc;
use core::sync::atomic::Ordering;

use crate::arch::ArchOps;
use crate::memory::addr::PhysAddr;
use crate::process::core::ProcessControlBlock;

pub(super) fn swap_address_space(pcb: &Arc<ProcessControlBlock>) -> Result<(), ()> {
    let root = pcb.cr3.load(Ordering::Relaxed);
    if root == 0 {
        return Ok(());
    }

    unsafe {
        <crate::arch::Arch as ArchOps>::switch_address_space(PhysAddr::new(root));
    }
    Ok(())
}
