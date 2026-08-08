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

use alloc::vec::Vec;
use nonos_libc::mk_ipc_call_timeout;

use super::connect::Conn;
use super::frame::{frame, status, HDR, OP_CLOSE, OP_SEND, SEG_MAX};

pub(super) const IO_MS: u64 = 6000;

impl Conn {
    pub fn send(&self, data: &[u8]) -> bool {
        let mut off = 0;
        while off < data.len() {
            let end = (off + SEG_MAX).min(data.len());
            let mut body = Vec::with_capacity(4 + (end - off));
            body.extend_from_slice(&self.handle.to_le_bytes());
            body.extend_from_slice(&data[off..end]);
            let tx = frame(OP_SEND, &body);
            let mut rx = [0u8; HDR];
            let n = mk_ipc_call_timeout(
                self.port as u64,
                tx.as_ptr(),
                tx.len(),
                rx.as_mut_ptr(),
                rx.len(),
                IO_MS,
            );
            if n < HDR as i64 || status(&rx) != 0 {
                return false;
            }
            off = end;
        }
        true
    }

    pub fn close(&self) {
        let tx = frame(OP_CLOSE, &self.handle.to_le_bytes());
        let mut rx = [0u8; HDR];
        let _ = mk_ipc_call_timeout(
            self.port as u64,
            tx.as_ptr(),
            tx.len(),
            rx.as_mut_ptr(),
            rx.len(),
            IO_MS,
        );
    }
}
