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

use super::header::{parse_response, write_request};
use super::seq;
use super::wire::{L2_HDR_LEN, OP_SEND_FRAME};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TxError {
    SendFailed,
    BadResponse,
    Refused(u16),
}

// Ship a fully-built ethernet frame down to L2 for TX. The
// frame must already include the 14-byte ethernet header
// (built by the IP capsule's egress path after ARP resolve).
pub fn send_frame(l2_port: u32, frame: &[u8]) -> Result<(), TxError> {
    let total = L2_HDR_LEN + frame.len();
    let mut req = vec![0u8; total];
    let rid = seq::next();
    write_request(&mut req, OP_SEND_FRAME, rid, frame.len() as u32);
    req[L2_HDR_LEN..total].copy_from_slice(frame);
    let mut resp = [0u8; L2_HDR_LEN];
    let n = mk_ipc_call(l2_port as u64, req.as_ptr(), total, resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(TxError::SendFailed);
    }
    let (op, errno, _, _) = parse_response(&resp).ok_or(TxError::BadResponse)?;
    if op != OP_SEND_FRAME {
        return Err(TxError::BadResponse);
    }
    if errno != 0 {
        return Err(TxError::Refused(errno));
    }
    Ok(())
}
