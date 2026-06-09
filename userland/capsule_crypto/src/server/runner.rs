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

use nonos_libc::{mk_ipc_recv, mk_ipc_send};

use super::dispatch::dispatch;
use super::wipe::wipe;
use crate::protocol::{
    decode_request, encode_response, EINVAL, HDR_LEN, KERNEL_REPLY_ENDPOINT, MAX_PAYLOAD_BYTES,
};

const MAX_MSG: usize = HDR_LEN + MAX_PAYLOAD_BYTES as usize;

pub fn run() -> ! {
    let mut buf = vec![0u8; MAX_MSG];
    loop {
        let n = mk_ipc_recv(0, buf.as_mut_ptr(), MAX_MSG, 0);
        if n <= 0 {
            continue;
        }
        let n = n as usize;
        let resp = match decode_request(&buf[..n]) {
            Ok(req) => dispatch(req),
            Err(_) => encode_response(0, 0, 0, EINVAL, &[]),
        };
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, resp.as_ptr(), resp.len());
        wipe(&mut buf[..n]);
    }
}
