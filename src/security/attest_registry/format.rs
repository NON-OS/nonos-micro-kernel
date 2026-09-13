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

//! The wire format of a registry receipt, in one place.
//!
//! Two programs fold these bytes and they are not in the same tree: this
//! kernel, producing the root a TPM signs, and whatever the person checking a
//! receipt is running. The reader in `userland/attest_receipt` compiles this
//! file directly rather than restating it, so a change to the domain or the
//! entry width breaks that build instead of silently making every receipt in
//! the field unverifiable against a machine that still looks healthy.
//!
//! Nothing else belongs here. The moment this file needs an import it stops
//! being includable from outside the kernel, and the second copy comes back.

/// Domain separator for the fold, so a registry root cannot be confused with
/// any other blake3 value this system produces, and so a digest lifted from
/// somewhere else cannot be presented as one.
pub const DOMAIN: &[u8] = b"nonos.attest.registry.v1";

/// Bytes per entry: pid (4, big endian), measurement (32), capability mask
/// (8, big endian), authority (1).
///
/// Fixed width rather than delimited, because a receipt is parsed by someone
/// who has no reason to trust the sender, and a length a sender chooses is a
/// length a sender can lie about. The count of entries is the byte length
/// divided by this, which is a fact about the bytes rather than a claim.
pub const ENTRY_LEN: usize = 45;
