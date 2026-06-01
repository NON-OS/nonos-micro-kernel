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

use nonos_libc::mk_ipc_call_timeout;
use nonos_policy_proto::{
    Field, Header, E_OK, HDR_LEN, IPC_PAYLOAD_MAX, KIND_U8, OP_GET,
};

const REPLY_TIMEOUT_MS: u64 = 200;

pub fn get_wallpaper(policy_port: u32) -> Option<u8> {
    let mut buf = [0u8; IPC_PAYLOAD_MAX];
    let req = Header {
        op: OP_GET,
        field: Field::Wallpaper as u32,
        kind: KIND_U8,
        status: 0,
        payload_len: 0,
    };
    req.encode(&mut buf[..HDR_LEN]);
    let n = mk_ipc_call_timeout(
        policy_port as u64,
        buf.as_ptr(),
        HDR_LEN,
        buf.as_mut_ptr(),
        buf.len(),
        REPLY_TIMEOUT_MS,
    );
    if n <= 0 || (n as usize) < HDR_LEN + 1 {
        return None;
    }
    let hdr = Header::decode(&buf[..HDR_LEN])?;
    if hdr.status != E_OK
        || hdr.op != OP_GET
        || hdr.kind != KIND_U8
        || hdr.field != Field::Wallpaper as u32
        || hdr.payload_len < 1
    {
        return None;
    }
    Some(buf[HDR_LEN])
}
