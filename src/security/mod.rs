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

extern crate alloc;

pub mod boot;
pub mod boot_session;
pub mod capsule_attest;
pub mod capsule_manifest;
pub mod crypto;
pub mod crypto_capsule;
pub mod entropy_capsule;
pub mod hardening;
#[cfg(feature = "nonos-stark-attest")]
pub mod kernel_attest;
pub mod keyring_capsule;
pub mod market_capsule;
pub mod module_db;
pub mod monitoring;
pub mod nonos_id_cert;
pub mod nonos_trust_anchor;
pub mod observability;
mod periodic;
pub mod policy;
pub mod quantum;
pub(crate) mod zerostate;

pub use boot::firmware;
pub use boot::secure_boot;

pub use boot::{
    firmware_init, generate_attestation_report, get_boot_measurements, get_policy,
    is_boot_chain_verified, is_enforcing, record_boot_measurements, secure_boot_init, set_policy,
    verify_boot_chain, verify_code_signature, verify_kernel, AttestationReport, BootMeasurements,
    FirmwareDB, SecureBootError, SecureBootPolicy, SecureBootResult, SecureBootStats,
    TrustedBootKeys,
};

pub use crypto::constant_time;
pub use crypto::key_management;
pub use crypto::random;
pub use crypto::trusted_hashes;
pub use crypto::trusted_hashes as nonos_trusted_hashes;
pub use crypto::trusted_keys;
pub use crypto::trusted_keys as nonos_trusted_keys;

pub use crypto::{
    add_trusted_hash, add_trusted_key, constant_time_init, ct_compare, ct_copy_bounded, ct_eq_u32,
    ct_eq_u64, ct_gt_u32, ct_hmac_verify, ct_lt_u32, ct_lt_u64, ct_max_u32, ct_min_u32,
    ct_select_slice, ct_select_u32, ct_select_u64, ct_select_u8, ct_signature_verify,
    ct_swap_slices, ct_verify, ct_zero, ct_zero_u64, delete_all_keys, delete_key, derive_key,
    export_key, fill_random, fill_random_bytes, find_key_by_fingerprint, generate_key,
    get_key_info, get_trusted_hash, get_trusted_key, get_trusted_keys, import_key,
    init_trusted_keys, key_management_init, list_keys, list_keys_by_owner, list_trusted_hashes,
    list_trusted_keys as crypto_list_trusted_keys, random_init, rotate_key, secure_random_u32,
    secure_random_u64, secure_random_u8, trusted_hashes_init, trusted_keys_init, use_key,
    verify_integrity, verify_signature as crypto_verify_signature, CtVerifyResult, KeyAuditEntry,
    KeyEntry, KeyError, KeyInfo, KeyOperation, KeyResult, KeyStore, KeyType, KeyUsage,
    SelfCheckResult, TimingMode, TrustedHashDB, TrustedKey, TrustedKeyDB,
};

pub use hardening::memory_sanitization;
#[cfg(target_arch = "x86_64")]
pub use hardening::spectre_mitigations;
pub use hardening::speculation;

#[cfg(target_arch = "x86_64")]
pub use hardening::{
    are_mitigations_enabled, array_access_nospec, array_index_mask_nospec,
    context_switch_mitigations, detect_vulnerabilities, enable_mitigations, get_mitigation_status,
    get_vulnerabilities, ibpb, ibrs_disable, ibrs_enable, kernel_entry_mitigations,
    kernel_exit_mitigations, l1d_flush, lfence, mds_clear, mfence, rsb_clear, rsb_fill, sfence,
    spectre_init, ssbd_disable, ssbd_enable, stibp_disable, stibp_enable, CpuVulnerabilities,
    MitigationStatus,
};

pub use hardening::{
    allocate_with_guards, dod_5220_erase, free_with_guards, get_level, get_stack_canary,
    gutmann_erase, init_stack_canary, memory_sanitization_init, on_free, on_realloc,
    paranoid_erase, sanitization_stats, sanitize, sanitize_process_memory, sanitize_slice,
    secure_zero, secure_zero_slice, set_level, stack_canary_failed, verify_stack_canary,
    zerostate_shutdown_wipe, GuardPage, SanitizationLevel, SanitizationStats, SecureString,
    SensitiveData, StackCanaryConfig,
};

pub use monitoring::audit;
pub use monitoring::monitor;
pub use monitoring::rootkit;

pub use monitoring::{
    audit_event, audit_init, clear_audit_log, get_audit_log, get_recent_events, is_enabled,
    log_event, log_security_event, log_security_violation, monitor_stats, rootkit_init,
    rootkit_last_scan, rootkit_scan, set_enabled, AuditEvent, AuditSeverity, MonitorStats,
    RootkitScanResult, SecurityAuditEvent, SecurityEvent, SecurityEventType,
};

pub use policy::advanced;
pub use policy::capability;
pub use policy::session;

pub use policy::advanced::*;
pub use policy::capability::*;
pub use policy::session::{
    chdir, current_cwd, current_uid, current_username, environ, get_stats as session_get_stats,
    getenv, init as session_init, session_manager, setenv, PrivilegeLevel, SessionManager,
    SessionState, SessionStats, UserAccount, UserSession, GID_ROOT, GID_USERS, GID_WHEEL,
    UID_ANONYMOUS, UID_DEFAULT, UID_ROOT,
};

pub use quantum::pqc;

pub use quantum::pqc::*;

pub use module_db::{get_loaded_modules, init as module_db_init, is_trusted_module, ModuleDB};

pub use periodic::run_periodic_checks;

pub use crate::usercopy;

pub use observability::{
    is_production_mode, redact_address, redact_panic_message, redact_pointer, serial_log,
    serial_log_redacted, set_production_mode, should_emit_serial, should_log_debug,
    ObservabilityPolicy, OutputMode,
};
