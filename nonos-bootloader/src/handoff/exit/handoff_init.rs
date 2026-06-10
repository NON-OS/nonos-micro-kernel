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

pub use super::params::HandoffInitParams;
use crate::handoff::types::{
    BootHandoffV1, CryptoHandoff, Measurements, MemoryMap, RngSeed, ZkAttestation, HANDOFF_MAGIC,
    HANDOFF_VERSION,
};
use core::mem::size_of;

/// Initialize handoff struct with boot parameters.
/// # Safety
/// bh_ptr must point to allocated memory of at least size_of::<BootHandoffV1>()
pub unsafe fn init_boothandoff(bh_ptr: *mut BootHandoffV1, p: &HandoffInitParams) {
    // Zero entire struct first to ensure no uninitialized padding bytes
    core::ptr::write_bytes(bh_ptr as *mut u8, 0, size_of::<BootHandoffV1>());
    (*bh_ptr).magic = HANDOFF_MAGIC;
    (*bh_ptr).version = HANDOFF_VERSION;
    (*bh_ptr).size = size_of::<BootHandoffV1>() as u16;
    (*bh_ptr).flags = p.handoff_flags;
    (*bh_ptr).entry_point = p.entry_point;
    (*bh_ptr).fb = p.fb_info;
    (*bh_ptr).mmap = MemoryMap { ptr: 0, entry_size: 0, entry_count: 0, desc_version: 0 };
    (*bh_ptr).acpi.rsdp = p.acpi_rsdp;
    (*bh_ptr).smbios.entry = p.smbios_entry;
    (*bh_ptr).modules.ptr = 0;
    (*bh_ptr).modules.count = 0;
    (*bh_ptr).modules.reserved = 0;
    (*bh_ptr).timing.tsc_hz = p.tsc_hz;
    (*bh_ptr).timing.unix_epoch_ms = p.unix_epoch_ms;
    init_measurements(bh_ptr, &p.crypto);
    init_rng_and_zk(bh_ptr, &p.crypto, p.rng_seed);
    (*bh_ptr).firmware = p.firmware;
    (*bh_ptr).cmdline_ptr = p.cmdline_addr;
}

unsafe fn init_measurements(bh_ptr: *mut BootHandoffV1, c: &CryptoHandoff) {
    (*bh_ptr).meas = Measurements {
        kernel_blake3: c.kernel_hash,
        kernel_sig_ok: c.signature_valid as u8,
        secure_boot: c.secure_boot as u8,
        zk_attestation_ok: c.zk_attested as u8,
        reserved: [0u8; 5],
    };
}

unsafe fn init_rng_and_zk(bh_ptr: *mut BootHandoffV1, c: &CryptoHandoff, seed: [u8; 32]) {
    (*bh_ptr).rng = RngSeed { seed32: seed };
    (*bh_ptr).zk = ZkAttestation {
        verified: c.zk_attested as u8,
        flags: 0,
        reserved: [0u8; 6],
        program_hash: c.zk_program_hash,
        capsule_commitment: c.zk_capsule_commitment,
    };
}
