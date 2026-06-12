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

pub const OP_HEALTHCHECK: u16 = 1;
pub const OP_INSTALL: u16 = 2;
pub const OP_LOAD_FROM_STORE: u16 = 3;

pub const KERNEL_REPLY_ENDPOINT: u64 = 0x1_0000_0011;

pub(super) const HDR_LEN: usize = 8;

pub const EINVAL: i32 = -22;
pub const EAGAIN: i32 = -11;

pub struct Request<'a> {
    pub seq: u32,
    pub op: u16,
    pub payload: &'a [u8],
}
