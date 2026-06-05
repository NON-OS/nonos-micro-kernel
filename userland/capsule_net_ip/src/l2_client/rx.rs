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

use super::header::{parse_response, write_request};
use super::seq;
use super::wire::{L2_HDR_LEN, OP_POLL_FRAME};

pub const MAX_FRAME: usize = 1514;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RxError {
    SendFailed,
    BadResponse,
    Empty,
    NoLink,
    Other(u16),
}

// Drain one inbound frame from `net.l2`. Returns owned bytes on
// success or `Empty` when the L2 NIC ring has nothing right now
// — caller backs off and tries again on the next tick.
pub fn poll_frame(l2_port: u32) -> Result<Vec<u8>, RxError> {
    let mut req = [0u8; L2_HDR_LEN];
    let rid = seq::next();
    write_request(&mut req, OP_POLL_FRAME, rid, 0);
    let mut resp = vec![0u8; L2_HDR_LEN + MAX_FRAME];
    let n = mk_ipc_call(l2_port as u64, req.as_ptr(), L2_HDR_LEN, resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(RxError::SendFailed);
    }
    let (op, errno, _, plen) = parse_response(&resp).ok_or(RxError::BadResponse)?;
    if op != OP_POLL_FRAME {
        return Err(RxError::BadResponse);
    }
    if errno == 8 {
        return Err(RxError::Empty);
    }
    if errno == 5 {
        return Err(RxError::NoLink);
    }
    if errno != 0 {
        return Err(RxError::Other(errno));
    }
    let want = L2_HDR_LEN + plen as usize;
    if want > resp.len() {
        return Err(RxError::BadResponse);
    }
    Ok(resp[L2_HDR_LEN..want].to_vec())
}
