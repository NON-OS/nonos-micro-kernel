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

//! The attest capsule's request, which is the cheapest reply on the machine.

/// The attest capsule's wire header: magic, version, op, then three words the
/// healthcheck leaves zero.
pub(super) const MAGIC: u32 = 0x4154_5354;
pub(super) const VERSION: u16 = 1;
pub(super) const OP_HEALTHCHECK: u16 = 0x0001;
pub(super) const HDR_LEN: usize = 20;
pub(super) const SERVICE: &[u8] = b"attest";

/// Generous against a single round trip, so a timeout means the peer is gone
/// rather than merely slow, and a run does not stall the terminal if it is.
pub(super) const TIMEOUT_MS: u64 = 200;
