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

use alloc::vec;

use nonos_libc::mk_ipc_call;

use super::header::{parse, write, HDR_LEN, OP_SEND};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UdpSendError {
    Send,
    BadResponse,
    Refused(u16),
}

pub fn send_to(
    udp_port: u32,
    local_port: u16,
    dst: [u8; 4],
    dst_port: u16,
    payload: &[u8],
) -> Result<(), UdpSendError> {
    let body_len = 8 + payload.len();
    let mut req = vec![0u8; HDR_LEN + body_len];
    write(&mut req, OP_SEND, 2, body_len as u32);
    req[HDR_LEN..HDR_LEN + 2].copy_from_slice(&local_port.to_le_bytes());
    req[HDR_LEN + 2..HDR_LEN + 6].copy_from_slice(&dst);
    req[HDR_LEN + 6..HDR_LEN + 8].copy_from_slice(&dst_port.to_le_bytes());
    req[HDR_LEN + 8..].copy_from_slice(payload);
    let mut resp = [0u8; HDR_LEN];
    let n = mk_ipc_call(udp_port as u64, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(UdpSendError::Send);
    }
    let (op, errno, _, _) = parse(&resp).ok_or(UdpSendError::BadResponse)?;
    if op != OP_SEND {
        return Err(UdpSendError::BadResponse);
    }
    if errno == 0 {
        Ok(())
    } else {
        Err(UdpSendError::Refused(errno))
    }
}
