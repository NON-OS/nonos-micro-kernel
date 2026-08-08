// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::super::constants::CANARY_MIX_CONSTANT;
use super::super::types::*;
use super::core::{MemoryHardening, MEMORY_HARDENING};
use crate::memory::addr::VirtAddr;
use crate::memory::{kaslr, layout};
use core::sync::atomic::Ordering;

impl MemoryHardening {
    pub(super) fn initialize(&self) -> Result<(), &'static str> {
        if self.initialized.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Ok(());
        }
        self.setup_kernel_guard_pages()?;
        self.setup_stack_protection()?;
        Ok(())
    }

    pub(super) fn setup_kernel_guard_pages(&self) -> Result<(), &'static str> {
        let kernel_sections = layout::kernel_sections();
        let mut guards = self.guard_pages.write();
        for section in &kernel_sections {
            if section.rx && !section.rw {
                let guard_before = GuardPage {
                    addr: VirtAddr::new(section.start.saturating_sub(layout::PAGE_SIZE as u64)),
                    size: layout::PAGE_SIZE,
                    protection_type: GuardType::KernelGuard,
                };
                let guard_after = GuardPage {
                    addr: VirtAddr::new(section.end),
                    size: layout::PAGE_SIZE,
                    protection_type: GuardType::KernelGuard,
                };
                guards.insert(guard_before.addr.as_u64(), guard_before);
                guards.insert(guard_after.addr.as_u64(), guard_after);
            }
        }
        Ok(())
    }

    pub(super) fn setup_stack_protection(&self) -> Result<(), &'static str> {
        let canary_value = self.generate_stack_canary();
        let stack_base = VirtAddr::new(layout::KHEAP_BASE - layout::KSTACK_SIZE as u64);
        let canary =
            StackCanary { value: canary_value, stack_base, stack_size: layout::KSTACK_SIZE };
        self.stack_canaries.write().insert(stack_base.as_u64(), canary);
        let guard_page = GuardPage {
            addr: VirtAddr::new(stack_base.as_u64().saturating_sub(layout::PAGE_SIZE as u64)),
            size: layout::PAGE_SIZE,
            protection_type: GuardType::StackGuard,
        };
        self.guard_pages.write().insert(guard_page.addr.as_u64(), guard_page);
        Ok(())
    }

    pub(super) fn generate_stack_canary(&self) -> u64 {
        let nonce = kaslr::boot_nonce().unwrap_or(0x1337DEADBEEF);
        let timestamp = crate::arch::read_time_counter();
        nonce.wrapping_mul(timestamp).wrapping_add(CANARY_MIX_CONSTANT)
    }
}

pub fn init() -> Result<(), &'static str> {
    MEMORY_HARDENING.initialize()
}
