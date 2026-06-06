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

pub(in super::super) const MAGIC: u32 = 0x4E4D_4B54;
pub(in super::super) const VERSION: u16 = 1;

pub(in super::super) const HDR_LEN: usize = 20;
pub(in super::super) const RESP_HDR_LEN: usize = HDR_LEN;

#[derive(Clone, Copy)]
pub struct Request {
    pub op: u16,
    pub flags: u16,
    pub request_id: u32,
    pub payload_len: u32,
}
