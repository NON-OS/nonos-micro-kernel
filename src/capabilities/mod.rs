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

mod bits;
pub mod roles;
mod serial_debug;
mod types;

pub mod audit;
pub mod chain;
pub mod delegation;
pub mod multisig;
pub mod resource;
pub mod token;

pub use audit::{
    capacity as audit_capacity, clear_log, get_by_action, get_by_capability, get_by_module,
    get_by_time_range, get_failures, get_log, get_recent, get_stats, get_successes, is_empty,
    log_count, log_failure, log_raw, log_success, log_use, reset_stats, AuditCounters, AuditEntry,
    AuditStatsSnapshot, BUFFER as AUDIT_BUFFER, MAX_LOG_ENTRIES, STATS as AUDIT_STATS,
};
pub use bits::{
    add_capability, bits_to_caps, capability_count, caps_to_bits, has_capability, remove_capability,
};
pub use chain::{
    effective_capabilities, first_invalid_index, is_chain_valid, max_chain_depth,
    verify_all_capabilities, verify_chain, verify_chain_capability, CapabilityChain, ChainError,
    MAX_CHAIN_DEPTH,
};
pub use delegation::{
    compute_delegation_signature, create_delegation, create_delegation_unchecked,
    delegation_material, sign_delegation, verify_delegation, verify_delegation_standalone,
    verify_delegation_strict, Delegation, DelegationError,
};
pub use multisig::{
    add_signature, clear_signatures, compute_signature as multisig_compute_signature,
    count_valid_signatures, create_multisig_token, create_multisig_token_with_nonce, max_signers,
    max_threshold, remove_signature, signature_material, verify_multisig, verify_multisig_strict,
    MultiSigError, MultiSigToken, MAX_SIGNERS, MAX_THRESHOLD, SIGNATURE_SIZE,
};
pub use resource::{
    compute_signature as resource_compute_signature, create_resource_token,
    create_resource_token_with_nonce, next_nonce as resource_next_nonce, refund_bytes, refund_ops,
    reset_nonce_counter as resource_reset_nonce_counter, reset_token, sign_resource_token,
    token_material as resource_token_material, try_consume, try_consume_bytes, try_consume_ops,
    verify_resource_token, verify_resource_token_strict, ResourceError, ResourceQuota,
    ResourceToken,
};
pub(crate) use serial_debug::serial_debug_cap;
pub use token::{
    clear_revocations, create_token, create_token_with_nonce, current_nonce_counter, default_nonce,
    from_bytes, has_signing_key, is_revoked, is_token_not_revoked, is_token_signature_valid,
    is_token_valid, mac64, reset_nonce_counter, revoke_all_for_owner, revoke_token, revoked_count,
    set_signing_key, sign_token, signing_key, to_bytes, token_material, validate_token_full,
    verify_token, CapabilityToken, TOKEN_BINARY_SIZE, TOKEN_VERSION,
};
pub use types::Capability;
