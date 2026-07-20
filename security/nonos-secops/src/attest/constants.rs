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

//! The attestation parameters the bootloader, the enrollment tool, and the
//! capsule gate all agree on. They are the single source of these numbers for
//! the security tools, so a tool cannot drift from the gate it tests.

pub const LOG_ROUNDS: u32 = 3;
pub const DEPTH: usize = 8;
pub const LEAVES: usize = 1 << DEPTH;
pub const N_QUERIES: usize = 32;
pub const GRIND_BITS: u32 = 16;
pub const EXTRA_BLOWUP_BITS: u32 = 3;
pub const BOOT_EPOCH: u64 = 1;
pub const PAD_IMAGE: &[u8] = b"\x00NONOS-POLICY-RESERVED-SLOT-v1";
