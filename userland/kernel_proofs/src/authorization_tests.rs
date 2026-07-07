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

use crate::capabilities::{bit_of, Capability, CapabilityToken};
use crate::syscall::contract::is_allowed;
use crate::syscall::numbers::SyscallNumber;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

// A token with a given set of capabilities and no expiry.
fn token(permissions: Vec<Capability>) -> CapabilityToken {
    CapabilityToken {
        owner_module: 0,
        permissions,
        expires_at_ms: None,
        nonce: 0,
        signature: [0u8; 64],
        token_id: 0,
        subject_capsule_id: 0,
        subject_asid: 0,
        subject_measurement: [0u8; 32],
        boot_session_nonce: [0u8; 16],
        revocation_epoch: 0,
        delegation_depth: 0,
    }
}

fn registered_syscalls() -> Vec<SyscallNumber> {
    (0u64..100_000).filter_map(SyscallNumber::from_u64).collect()
}

#[test]
fn an_empty_token_is_denied_every_syscall() {
    let empty = token(vec![]);
    for n in registered_syscalls() {
        assert!(!is_allowed(&empty, n), "an empty capability token must be denied every syscall");
    }
}

#[test]
fn crypto_syscalls_require_the_crypto_capability() {
    let with_crypto = token(vec![Capability::Crypto]);
    let without_crypto = token(vec![Capability::Hardware]);

    assert!(is_allowed(&with_crypto, SyscallNumber::CryptoRandom));
    assert!(is_allowed(&with_crypto, SyscallNumber::CryptoHash));
    assert!(!is_allowed(&without_crypto, SyscallNumber::CryptoRandom));
    assert!(!is_allowed(&without_crypto, SyscallNumber::CryptoHash));
}

#[test]
fn granting_a_capability_never_removes_access() {
    // Monotonicity: a superset of capabilities allows everything the subset does.
    let subset = token(vec![Capability::Crypto]);
    let superset = token(vec![Capability::Crypto, Capability::Admin, Capability::Memory]);
    for n in registered_syscalls() {
        if is_allowed(&subset, n) {
            assert!(is_allowed(&superset, n), "adding a capability removed access");
        }
    }
}

#[test]
fn capability_bits_are_distinct_single_bits() {
    // The gate ultimately rests on each capability occupying its own bit.
    let caps = [
        Capability::CoreExec,
        Capability::IO,
        Capability::Network,
        Capability::IPC,
        Capability::Memory,
        Capability::Crypto,
        Capability::FileSystem,
        Capability::Hardware,
        Capability::Debug,
        Capability::Admin,
    ];
    for c in caps {
        assert_eq!(bit_of(c).count_ones(), 1, "each capability must be a single bit");
    }
    for i in 0..caps.len() {
        for j in (i + 1)..caps.len() {
            assert_ne!(bit_of(caps[i]), bit_of(caps[j]), "two capabilities share a bit");
        }
    }
}
