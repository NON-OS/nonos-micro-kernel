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

pub const DOM_PEDERSEN: &[u8] = b"NONOS:TRANSPARENT:PEDERSEN:v1";

pub const DOM_RANGE: &[u8] = b"NONOS_ZK_RANGE_V1";

pub const DOM_EQUALITY: &[u8] = b"NONOS_ZK_EQUALITY_V1";

pub const DOM_MERKLE: &[u8] = b"NONOS_ZK_MERKLE_V1";

pub const DOM_PLONK: &[u8] = b"NONOS_ZK_PLONK_V1";

pub(crate) const DOM_ATTEST: &[u8] = b"NONOS:TRANSPARENT:ATTEST:v1";

pub(crate) const DOM_MERKLE_LEAF: &[u8] = b"NONOS:TRANSPARENT:MERKLE:LEAF:v1";

pub(crate) const DOM_MERKLE_NODE: &[u8] = b"NONOS:TRANSPARENT:MERKLE:NODE:v1";

pub const L: [u8; 32] = [
    0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x1e, 0x8d, 0xce, 0x4c, 0xcd, 0x65, 0xa0, 0x2f, 0x8a, 0x01,
    0x4f, 0xd9, 0x12, 0x6b, 0x83, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
];
