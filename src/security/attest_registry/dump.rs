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

//! The entries behind the root, in the order the root folds them.
//!
//! [`registry_root`](super::registry_root) is what an attestation binds, and
//! on its own it is an opaque digest. A verifier handed a TPM-signed root
//! learns that the machine asserted some value; it cannot learn which programs
//! are running, and it cannot check that the value corresponds to anything at
//! all. That makes the strongest claim this system can make unstateable: that
//! the program which handled something was not permitted to reach the network.
//!
//! These bytes close that. A verifier recomputes the root from them, exactly
//! as `registry_root` does, and compares it to the one the TPM signed. If it
//! matches, the entries are authentic, and their capability masks say what
//! each program was permitted to do.
//!
//! Nothing here has to be trusted, which is the point. The entries are
//! self-authenticating against a signature the caller cannot forge, so handing
//! them out over any channel grants no authority and adds no assumption. A
//! caller that alters one produces a set that no longer folds to the signed
//! root, and the alteration is what the verifier detects.

use alloc::vec::Vec;

use super::format::ENTRY_LEN;
use super::order::sorted_order;
use super::table::TABLE;

/// Every recorded capsule, in the root's order, each as its `digest_input`.
///
/// The order comes from [`sorted_order`], the same function the root folds
/// through, so the two cannot disagree. Emitting these in table order instead
/// would produce a set that is correct and unverifiable.
///
/// The count is not emitted. It is `len() / ENTRY_LEN`, and a verifier that
/// takes the count from the bytes it is checking rather than from a field it
/// was handed cannot be lied to about how much is running.
pub fn registry_entries() -> Vec<u8> {
    let table = TABLE.lock();
    let (order, used) = sorted_order(&table);

    let mut out = Vec::with_capacity(used * ENTRY_LEN);
    for &i in &order[..used] {
        out.extend_from_slice(&table.entries[i].digest_input());
    }
    out
}
