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

use nonos_libc::mk_ipc_call;

use super::header::{parse, write, HDR_LEN, OP_BIND};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UdpBindError {
    Send,
    BadResponse,
    Refused(u16),
}

pub fn bind(udp_port: u32, local_port: u16) -> Result<(), UdpBindError> {
    let mut req = [0u8; HDR_LEN + 2];
    write(&mut req, OP_BIND, 1, 2);
    req[HDR_LEN..HDR_LEN + 2].copy_from_slice(&local_port.to_le_bytes());
    let mut resp = [0u8; HDR_LEN];
    let n = mk_ipc_call(udp_port as u64, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(UdpBindError::Send);
    }
    let (op, errno, _, _) = parse(&resp).ok_or(UdpBindError::BadResponse)?;
    if op != OP_BIND {
        return Err(UdpBindError::BadResponse);
    }
    if errno == 0 || errno == 6 {
        Ok(())
    } else {
        Err(UdpBindError::Refused(errno))
    }
}
