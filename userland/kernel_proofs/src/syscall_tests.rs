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

use crate::syscall::abi::lookup_name;
use crate::syscall::numbers::SyscallNumber;

// A syscall id crosses the kernel boundary as an untrusted u64. Decoding it must
// be total (never panic on any value), and the id/variant table must stay
// consistent with the name table.

#[test]
fn decode_is_total_and_agrees_with_the_name_table() {
    for id in 0u64..100_000 {
        // A registered id decodes to a variant exactly when it has a name.
        assert_eq!(SyscallNumber::from_u64(id).is_some(), lookup_name(id).is_some());
    }
    // Extreme values decode to None without panicking.
    for &id in &[u64::MAX, u64::MAX - 1, 1u64 << 63, 0x8000_0000_0000_0001] {
        assert!(SyscallNumber::from_u64(id).is_none());
    }
}

#[test]
fn decode_is_deterministic() {
    for id in 0u64..5_000 {
        assert_eq!(SyscallNumber::from_u64(id), SyscallNumber::from_u64(id));
    }
}

#[test]
fn known_syscalls_round_trip_through_their_numeric_id() {
    // The enum is `repr(u64)` with `tag4` discriminants, so `variant as u64` is
    // the ABI id the boundary receives; decoding it must recover the variant.
    for v in [SyscallNumber::CryptoRandom, SyscallNumber::CryptoHash] {
        assert_eq!(SyscallNumber::from_u64(v as u64), Some(v));
        assert!(lookup_name(v as u64).is_some());
    }
}
