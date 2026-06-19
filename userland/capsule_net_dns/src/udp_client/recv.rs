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
use alloc::vec::Vec;

use nonos_libc::mk_ipc_call;

use super::header::{parse, write, HDR_LEN, OP_RECV};

const UDP_E_RX_EMPTY: u16 = 8;

#[derive(Clone, PartialEq, Eq)]
pub struct UdpDatagram {
    pub src: [u8; 4],
    pub src_port: u16,
    pub payload: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UdpRecvError {
    Send,
    BadResponse,
    Empty,
    Refused(u16),
}

pub fn recv_from(udp_port: u32, local_port: u16) -> Result<UdpDatagram, UdpRecvError> {
    let mut req = [0u8; HDR_LEN + 2];
    write(&mut req, OP_RECV, 3, 2);
    req[HDR_LEN..HDR_LEN + 2].copy_from_slice(&local_port.to_le_bytes());
    let mut resp = vec![0u8; HDR_LEN + 6 + 512];
    let n = mk_ipc_call(udp_port as u64, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(UdpRecvError::Send);
    }
    let (op, errno, _, len) = parse(&resp).ok_or(UdpRecvError::BadResponse)?;
    if op != OP_RECV {
        return Err(UdpRecvError::BadResponse);
    }
    if errno == UDP_E_RX_EMPTY {
        return Err(UdpRecvError::Empty);
    }
    if errno != 0 || len < 6 {
        return Err(UdpRecvError::Refused(errno));
    }
    let end = HDR_LEN + len as usize;
    if end > resp.len() {
        return Err(UdpRecvError::BadResponse);
    }
    let mut src = [0u8; 4];
    src.copy_from_slice(&resp[HDR_LEN..HDR_LEN + 4]);
    let src_port = u16::from_le_bytes([resp[HDR_LEN + 4], resp[HDR_LEN + 5]]);
    Ok(UdpDatagram { src, src_port, payload: resp[HDR_LEN + 6..end].to_vec() })
}
