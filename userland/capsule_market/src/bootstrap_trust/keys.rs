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

pub(super) const NOX_OPERATOR_V1: [u8; 32] = [
    0x29, 0x5f, 0x84, 0xc9, 0x7c, 0x62, 0x01, 0x3c, 0x43, 0x8b, 0xca, 0x3d, 0x81, 0xc1, 0x80, 0x98,
    0x1b, 0x9f, 0x0a, 0x04, 0x3b, 0xa1, 0xfa, 0xe2, 0x54, 0xad, 0x0e, 0x12, 0xea, 0x8e, 0x07, 0x63,
];

#[cfg(feature = "smoketest-trust")]
pub(super) const SMOKETEST_OPERATOR: [u8; 32] = [
    0x21, 0x52, 0xf8, 0xd1, 0x9b, 0x79, 0x1d, 0x24, 0x45, 0x32, 0x42, 0xe1, 0x5f, 0x2e, 0xab, 0x6c,
    0xb7, 0xcf, 0xfa, 0x7b, 0x6a, 0x5e, 0xd3, 0x00, 0x97, 0x96, 0x0e, 0x06, 0x98, 0x81, 0xdb, 0x12,
];

#[cfg(not(feature = "smoketest-trust"))]
pub(super) const TRUSTED_OPERATORS: &[[u8; 32]] = &[NOX_OPERATOR_V1];

#[cfg(feature = "smoketest-trust")]
pub(super) const TRUSTED_OPERATORS: &[[u8; 32]] = &[NOX_OPERATOR_V1, SMOKETEST_OPERATOR];
