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
use crate::ipc::mk_ipc_recv;
use crate::transport::error::TransportError;

const RECV_OWN_INBOX: u64 = 0;

pub fn recv_response(out_buf: &mut [u8], timeout_ms: u64) -> Result<usize, TransportError> {
    let n = mk_ipc_recv(RECV_OWN_INBOX, out_buf.as_mut_ptr(), out_buf.len(), timeout_ms);
    if n < 0 {
        Err(TransportError::RecvTimeout)
    } else {
        Ok(n as usize)
    }
}
