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

pub const P_HEX: &str = "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47";

pub const R_HEX: &str = "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001";

pub const SECURITY_BITS: u32 = 100;

pub const PROOF_SIZE_UNCOMPRESSED: usize = 256;

pub const PROOF_SIZE_COMPRESSED: usize = 128;

pub const MIN_VK_SIZE_COMPRESSED: usize = 32 + 64 + 64 + 64 + 32;

pub const G1_COMPRESSED_SIZE: usize = 32;

pub const G2_COMPRESSED_SIZE: usize = 64;

pub const G1_UNCOMPRESSED_SIZE: usize = 64;

pub const G2_UNCOMPRESSED_SIZE: usize = 128;

pub const MAX_VK_BYTES: usize = 16 * 1024 * 1024;

pub const MAX_PROOF_BYTES: usize = 1024 * 1024;

pub const MAX_PUBLIC_INPUTS: usize = 262_000;
