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

pub const DOM_ATTEST: &[u8] = b"NONOS:TRANSPARENT:ATTEST:v1";
pub const DOM_MERKLE_LEAF: &[u8] = b"NONOS:TRANSPARENT:MERKLE:LEAF:v1";
pub const DOM_MERKLE_NODE: &[u8] = b"NONOS:TRANSPARENT:MERKLE:NODE:v1";
pub const DOM_PEDERSEN: &[u8] = b"NONOS:TRANSPARENT:PEDERSEN:v1";
pub const GENERATOR_H_LABEL: &[u8] = b"generator_h";
pub const STATIC_BOOT_PUBLIC_INPUTS_LEN: usize = 32;
pub const RUNTIME_BOOT_PUBLIC_INPUTS_LEN: usize = 104;
pub const FIXED_PROOF_BYTES: usize = 129;
