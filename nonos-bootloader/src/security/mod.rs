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

pub mod anti_rollback;
pub mod attestation;
pub mod audit;
mod check;
#[cfg(target_arch = "x86_64")]
mod cpuid;
mod crypto;
mod enforce;
pub mod hardware;
mod init;
pub mod integrity;
pub mod memory;
pub mod root_of_trust;
pub mod timing;
mod tpm_extend;
mod tpm_nv;
mod tpm_types;
mod types;
mod verify;

pub use anti_rollback::{
    check_kernel_version, init_anti_rollback, update_kernel_version, RollbackError,
};
pub use attestation::{
    generate_attestation_quote, init_attestation, set_bootloader_measurement,
    set_kernel_measurement, set_signature_attestation, set_zk_attestation,
    verify_attestation_quote, AttestationQuote,
};
pub use audit::{
    audit, audit_alert, get_audit_hash, seal_audit_log, verify_audit_integrity, AuditEvent,
};
pub use check::{
    check_hardware_rng, check_measured_boot, check_platform_key, check_secure_boot,
    check_signature_db,
};
pub use crypto::{blake3_health_check, ed25519_health_check, run_all_crypto_health_checks};
pub use enforce::{
    detect_secure_boot_bypass, enforce_security_policy, extend_boot_measurements,
    verify_kernel_version, verify_secure_boot_chain, EnforcementResult, SecurityPolicy,
};
pub use hardware::{
    check_minimum_requirements, check_recommended_requirements, detect_hardware_capabilities,
    verify_platform_security, HardwareCapabilities, PlatformVerification, RequirementCheck,
};
pub use init::{assess_security_posture, initialize_security_subsystem};
pub use memory::{
    init_canaries, verify_heap_canary, verify_stack_canary, zeroize_slice, SecureBuffer, SecureKey,
};
pub use tpm_extend::{extend_pcr_measurement, measure_boot_components};
pub use tpm_nv::{commit_floor, read_floor, tpm_counter_selftest, RC_NO_TPM};
pub use tpm_types::{EV_POST_CODE, PCR_BOOTLOADER, PCR_CAPSULE, PCR_KERNEL};
pub use types::SecurityContext;
pub use verify::{verify_kernel_signature_advanced, verify_signature};
