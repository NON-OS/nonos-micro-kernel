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

use nonos_policy_proto::{Header, HDR_LEN, IPC_PAYLOAD_MAX};

use super::super::reply;

pub fn err(pid: u32, op: u16, field: u32, kind: u8, status: u16) {
    let hdr = Header { op, field, kind, status, payload_len: 0 };
    let mut buf = [0u8; IPC_PAYLOAD_MAX];
    hdr.encode(&mut buf[..HDR_LEN]);
    reply::send(pid, &buf[..HDR_LEN]);
}
