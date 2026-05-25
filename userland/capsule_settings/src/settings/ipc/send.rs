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

use nonos_libc::mk_ipc_send;
use nonos_policy_proto::{Header, HDR_LEN, IPC_PAYLOAD_MAX};

use super::error::IpcError;

pub fn send(port: u32, op: u16, field: u32, kind: u8, payload: &[u8]) -> Result<(), IpcError> {
    let mut buf = [0u8; IPC_PAYLOAD_MAX];
    let hdr = Header { op, field, kind, status: 0, payload_len: payload.len() as u16 };
    hdr.encode(&mut buf[..HDR_LEN]);
    let total = HDR_LEN + payload.len();
    buf[HDR_LEN..total].copy_from_slice(payload);
    let rc = mk_ipc_send(port as u64, buf.as_ptr(), total);
    if rc < 0 {
        return Err(IpcError::SendFailed);
    }
    Ok(())
}
