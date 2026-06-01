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
use nonos_policy_proto::{Header, E_OK, HDR_LEN, IPC_PAYLOAD_MAX};

use super::error::IpcError;
use super::timeout::REPLY_TIMEOUT_MS;

pub struct Reply<'a> {
    pub header: Header,
    pub payload: &'a [u8],
}

pub fn call<'a>(
    port: u32,
    op: u16,
    field: u32,
    kind: u8,
    req_payload: &[u8],
    rx: &'a mut [u8; IPC_PAYLOAD_MAX],
) -> Result<Reply<'a>, IpcError> {
    if req_payload.len() > IPC_PAYLOAD_MAX - HDR_LEN || req_payload.len() > u16::MAX as usize {
        return Err(IpcError::SendFailed);
    }
    let mut tx = [0u8; IPC_PAYLOAD_MAX];
    let hdr = Header {
        op,
        field,
        kind,
        status: 0,
        payload_len: req_payload.len() as u16,
    };
    hdr.encode(&mut tx[..HDR_LEN]);
    let total = HDR_LEN + req_payload.len();
    tx[HDR_LEN..total].copy_from_slice(req_payload);
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        total,
        rx.as_mut_ptr(),
        rx.len(),
        REPLY_TIMEOUT_MS,
    );
    if n <= 0 {
        return Err(IpcError::RecvTimeout);
    }
    let total = n as usize;
    if total < HDR_LEN {
        return Err(IpcError::ShortReply);
    }
    let header = Header::decode(&rx[..HDR_LEN]).ok_or(IpcError::BadHeader)?;
    let body_end = HDR_LEN + header.payload_len as usize;
    if body_end > total {
        return Err(IpcError::ShortReply);
    }
    if header.status != E_OK {
        return Err(IpcError::Status(header.status));
    }
    Ok(Reply { header, payload: &rx[HDR_LEN..body_end] })
}
