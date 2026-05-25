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

use nonos_libc::mk_ipc_recv;
use nonos_policy_proto::{Header, E_OK, HDR_LEN, IPC_PAYLOAD_MAX};

use super::error::IpcError;
use super::timeout::REPLY_TIMEOUT_MS;

const OWN_INBOX: u64 = 0;

pub struct Reply<'a> {
    pub header: Header,
    pub payload: &'a [u8],
}

pub fn recv_into<'a>(buf: &'a mut [u8; IPC_PAYLOAD_MAX]) -> Result<Reply<'a>, IpcError> {
    let n = mk_ipc_recv(OWN_INBOX, buf.as_mut_ptr(), buf.len(), REPLY_TIMEOUT_MS);
    if n <= 0 {
        return Err(IpcError::RecvTimeout);
    }
    let total = n as usize;
    if total < HDR_LEN {
        return Err(IpcError::ShortReply);
    }
    let header = Header::decode(&buf[..HDR_LEN]).ok_or(IpcError::BadHeader)?;
    let body_end = HDR_LEN + header.payload_len as usize;
    if body_end > total {
        return Err(IpcError::ShortReply);
    }
    if header.status != E_OK {
        return Err(IpcError::Status(header.status));
    }
    Ok(Reply { header, payload: &buf[HDR_LEN..body_end] })
}
