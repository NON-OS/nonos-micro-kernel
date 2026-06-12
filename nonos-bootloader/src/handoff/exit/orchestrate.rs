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

use uefi::prelude::*;

use super::cleanup::secure_cleanup_before_jump;
use super::gather::gather_system_info;
use super::handoff_init::init_boothandoff;
use super::validate::{validate_and_jump, JumpAddresses};
use crate::arch::x86_64::serial::com1_marker;
use crate::firmware::FirmwareHandoff;
use crate::handoff::jump::{copy_memory_map, finalize_mmap, settle_delay};
use crate::handoff::prepare::allocate_handoff_resources;
use crate::handoff::prepare::fatal_alloc_error;
use crate::handoff::types::{BootHandoffV1, CryptoHandoff};
use crate::loader::KernelImage;
use crate::paging::{build_kernel_pml4, phys_to_directmap_virt, switch_to_kernel_pml4};

pub fn exit_and_jump(
    st: SystemTable<Boot>,
    kernel: &KernelImage,
    cmdline: Option<&str>,
    crypto: CryptoHandoff,
    firmware: FirmwareHandoff,
    rng_seed: [u8; 32],
    tpm_measured: bool,
) -> ! {
    let bs = st.boot_services();
    let allocs = allocate_handoff_resources(&st, cmdline);
    let params =
        gather_system_info(&st, bs, kernel, &crypto, firmware, tpm_measured, &allocs, rng_seed);
    let bh_ptr = allocs.boothandoff_addr as *mut BootHandoffV1;
    unsafe { init_boothandoff(bh_ptr, &params) };

    // Build the kernel paging contract while UEFI Boot Services
    // can still hand us page-table frames. The new PML4 carries
    // a low identity range over [0, IDENTITY_LOW_BYTES) (so
    // bootloader text/data, the loaded kernel ELF, the handoff
    // struct, the boot stack, the memory map area, and the
    // framebuffer all stay reachable through the CR3 swap — the
    // window is sized to reach firmware image loads above 4 GiB)
    // plus a 256-GiB linear directmap at
    // PML4[256] (the `phys_to_virt` window the kernel asserts
    // on first VM init).
    com1_marker(b"PT0");
    let new_pml4 = match build_kernel_pml4(bs, kernel) {
        Ok(p) => p,
        Err(e) => fatal_alloc_error(&st, e),
    };
    com1_marker(b"PT1");
    if kernel.is_upper_half() {
        com1_marker(b"KTXT");
    }

    // Pass stack and handoff as directmap virts, not phys. The
    // identity-low window survives the CR3 swap but the kernel
    // tears it down after the directmap probe; after that, only
    // PML4[256..511] is reachable. Directmap virts work in both
    // worlds.
    let stack_va = match phys_to_directmap_virt(allocs.stack_top as u64) {
        Ok(v) => v,
        Err(e) => fatal_alloc_error(&st, e),
    };
    let handoff_va = match phys_to_directmap_virt(allocs.boothandoff_addr) {
        Ok(v) => v,
        Err(e) => fatal_alloc_error(&st, e),
    };

    crate::display::gop::shutdown_for_exit();
    secure_cleanup_before_jump();
    settle_delay();
    com1_marker(b"EBS0");
    let (_runtime_st, final_mmap) = st.exit_boot_services();
    com1_marker(b"EBS1");
    let (mmap_ptr, entry_size, entry_count) = copy_memory_map(allocs.mmap_addr, &final_mmap);
    finalize_mmap(bh_ptr, mmap_ptr, entry_size, entry_count);
    com1_marker(b"MMAP1");

    // Switch CR3 to the bootloader-built PML4 immediately before
    // transferring control. After this write, only mappings
    // present in the new PML4 are reachable; the identity-mapped
    // low region keeps the bootloader's running code valid
    // through these last instructions.
    unsafe { switch_to_kernel_pml4(new_pml4) };
    com1_marker(b"CR3OK");

    validate_and_jump(JumpAddresses {
        entry: kernel.entry_point as u64,
        stack: stack_va,
        handoff: handoff_va,
    })
}
