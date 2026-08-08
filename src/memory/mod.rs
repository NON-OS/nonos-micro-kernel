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

// Canonical memory authority. Owns physical/virtual memory, paging,
// MMU, KASLR, encryption, hardening, secure memory, DMA, frame
// allocator, and unified VM init. The unified entry points are
// `crate::memory::unified::{init_all_memory_subsystems, init_unified_vm}`.
// The live global allocator is `crate::memory::heap::manager::globals`.
//
// `nonos_*` aliases are retained because external consumers still
// resolve through them (drivers/i2c, storage/{ahci,nvme}, smp/init).
// Dropping the prefixed aliases is a later narrowing pass.

extern crate alloc;

mod api;

pub mod addr;
pub mod boot_memory;
pub mod buddy_alloc;
pub mod dma;
// AMD SME/SEV and Intel TME/MKTME, detected through CPUID and driven through
// model-specific registers. ARM's equivalent is CCA/RME, which is a different
// mechanism with a different trust model, not a port of this one.
#[cfg(target_arch = "x86_64")]
pub mod encryption;
pub mod frame_alloc;
pub mod hardening;
pub mod heap;
// The backend inside picks VT-d or the unsupported stand-in by target and
// feature, so this builds anywhere. An ARM board has no SMMU driver behind it
// yet, which means every mapping request is refused rather than ignored.
pub mod iommu;
pub mod kaslr;
pub mod layout;
pub mod mmio;
#[cfg(target_arch = "x86_64")]
pub mod mmu;
pub mod page_allocator;
pub mod page_info;
pub mod paging;
pub mod phys;
pub mod proof;
pub mod region;
pub mod safety;
pub mod secure_memory;
pub mod stats;
pub mod unified;

pub use addr::{PhysAddr, VirtAddr};
pub use api::{get_memory_stats, get_process_vm_areas, read_process_memory};
pub use buddy_alloc as allocator;
pub use frame_alloc as nonos_frame_alloc;
pub use hardening::{
    get_all_process_regions, init_module_memory_protection, read_bytes,
    verify_kernel_data_integrity, verify_kernel_page_tables,
};
pub use iommu::{DeviceAddress, DomainId, IommuDomain, IommuError, IommuProtection};
pub use layout as nonos_layout;
#[cfg(target_arch = "x86_64")]
pub use paging as nonos_paging;
pub use secure_memory as memory;
pub use unified::{
    allocate_secure_region, flush_tlb_all, flush_tlb_range, get_memory_system_stats,
    get_unified_vm_stats, handle_unified_page_fault, init_all_memory_subsystems, init_unified_vm,
    is_address_mapped, map_memory, phys_to_virt, translate_virtual, unmap_memory, validate_access,
    verify_all_memory_integrity, virt_to_phys, MemoryProtection, MemorySystemStats, MemoryType,
    UnifiedVmStats,
};
