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

//! A program the machine installed for itself.

use nonos_libc::{mk_local_verify, CapsuleVerifySummary};

/// A guest is spawned holding nothing, so the proof that admits it is a proof
/// for nothing.
const GUEST_CAPS: u64 = 0;

pub fn verify(image: &[u8], trailer: &[u8]) -> Result<CapsuleVerifySummary, &'static str> {
    if !mk_local_verify(image, GUEST_CAPS, trailer) {
        return Err("no publisher, and the machine did not vouch for it");
    }
    Ok(CapsuleVerifySummary::zeroed())
}
