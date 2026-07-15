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

pub const OP_HEALTHCHECK: u16 = 0x0001;
pub const OP_PROBE_CONFIG: u16 = 0x0002;
pub const OP_BUILD_INQUIRY: u16 = 0x0003;
pub const OP_BUILD_READ_CAPACITY10: u16 = 0x0004;
pub const OP_BUILD_READ10: u16 = 0x0005;
pub const OP_BUILD_WRITE10: u16 = 0x0006;
pub const OP_ACCEPT_CSW: u16 = 0x0007;
pub const OP_GET_STATE: u16 = 0x0008;
pub const OP_BUILD_TEST_UNIT_READY: u16 = 0x0009;
pub const OP_BUILD_REQUEST_SENSE: u16 = 0x000A;
pub const OP_DECODE_INQUIRY: u16 = 0x000B;
pub const OP_DECODE_CAPACITY: u16 = 0x000C;
pub const OP_DECODE_SENSE: u16 = 0x000D;
