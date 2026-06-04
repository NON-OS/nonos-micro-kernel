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

use crate::command::wire::{encode_header, HDR_LEN, OP_LIST_APPS};

pub(super) fn call_list(port: u32, rsp: &mut [u8]) -> Result<usize, i64> {
    let mut req = [0u8; HDR_LEN];
    encode_header(&mut req, OP_LIST_APPS, 0);
    let n = mk_ipc_call(port as u64, req.as_ptr(), req.len(), rsp.as_mut_ptr(), rsp.len());
    if n > 0 {
        Ok(n as usize)
    } else {
        Err(n)
    }
}
