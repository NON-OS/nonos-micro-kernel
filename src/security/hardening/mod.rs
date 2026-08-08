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

pub mod memory_encryption;
pub mod memory_sanitization;
pub mod speculation;
// Every symbol below is an x86_64 mitigation with no counterpart elsewhere:
// model-specific registers and retpoline thunks named after x86 registers.
// Shared code goes through `speculation` instead.
#[cfg(target_arch = "x86_64")]
pub mod spectre_mitigations;

#[cfg(target_arch = "x86_64")]
pub use spectre_mitigations::{
    __x86_indirect_thunk_rax, __x86_indirect_thunk_rbx, __x86_indirect_thunk_rcx,
    __x86_indirect_thunk_rdx, __x86_indirect_thunk_rsi, are_mitigations_enabled,
    array_access_nospec, array_index_mask_nospec, context_switch_mitigations,
    detect_vulnerabilities, enable_mitigations, get_mitigation_status, get_vulnerabilities, ibpb,
    ibrs_disable, ibrs_enable, init as spectre_init, kernel_entry_mitigations,
    kernel_exit_mitigations, l1d_flush, lfence, mds_clear, mfence, rsb_clear, rsb_fill, sfence,
    ssbd_disable, ssbd_enable, stibp_disable, stibp_enable, CpuVulnerabilities, MitigationStatus,
    __x86_indirect_thunk_r8, __x86_indirect_thunk_rdi,
};

pub use memory_sanitization::{
    allocate_with_guards, dod_5220_erase, free_with_guards, get_level, get_stack_canary,
    gutmann_erase, init as memory_sanitization_init, init_stack_canary, on_free, on_realloc,
    paranoid_erase, sanitization_stats, sanitize, sanitize_process_memory, sanitize_slice,
    secure_zero, secure_zero_slice, set_level, stack_canary_failed, verify_stack_canary,
    zerostate_shutdown_wipe, GuardPage, SanitizationLevel, SanitizationStats, SecureString,
    SensitiveData, StackCanaryConfig,
};

pub use memory_encryption::{
    decrypt_region, encrypt_region, get_protected_regions, init as memory_encryption_init,
    is_initialized as mem_encrypt_initialized, is_region_protected, protect_sensitive,
    register_region as register_encrypted_region, rotate_keys as rotate_encryption_keys,
    unprotect_sensitive, unregister_region as unregister_encrypted_region, EncryptedRegion,
    EncryptionError, MemEncryptStats,
};
