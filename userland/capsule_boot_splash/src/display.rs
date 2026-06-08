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

use crate::proto::{header, HDR_LEN};

const OP_DISPLAY_INFO: u16 = 0x0008;
const RESP_LEN: usize = HDR_LEN + 4 + 16;

fn read_u32(rx: &[u8], off: usize) -> Result<u32, i32> {
    Ok(u32::from_le_bytes(rx[off..off + 4].try_into().map_err(|_| -11)?))
}

pub(crate) fn query(port: u32, rid: u32) -> Result<(u32, u32, u32), i32> {
    let tx = header(OP_DISPLAY_INFO, rid, 0);
    let mut rx = vec![0u8; RESP_LEN];
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < RESP_LEN as i64 {
        return Err(-11);
    }
    if read_u32(&rx, HDR_LEN)? as i32 != 0 {
        return Err(-1);
    }
    let width = read_u32(&rx, HDR_LEN + 4)?;
    let height = read_u32(&rx, HDR_LEN + 8)?;
    let stride = read_u32(&rx, HDR_LEN + 12)?;
    if width == 0 || height == 0 || stride == 0 {
        return Err(-22);
    }
    Ok((width, height, stride))
}
