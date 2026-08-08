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
use nonos_libc::mk_ipc_call_timeout;

use super::connect::{Conn, Rx};
use super::frame::{frame, payload_len, status, HDR, OP_RECV, OP_STATE, RX_CAP};
use super::io_send::IO_MS;

const E_RX_EMPTY: u16 = 11;

impl Conn {
    pub fn recv(&self, out: &mut Vec<u8>) -> Rx {
        let tx = frame(OP_RECV, &self.handle.to_le_bytes());
        let mut rx = vec![0u8; RX_CAP];
        let n = mk_ipc_call_timeout(
            self.port as u64,
            tx.as_ptr(),
            tx.len(),
            rx.as_mut_ptr(),
            rx.len(),
            IO_MS,
        );
        if n < HDR as i64 {
            return Rx::Gone;
        }
        match status(&rx) {
            0 => {
                let plen = payload_len(&rx).min(n as usize - HDR);
                out.extend_from_slice(&rx[HDR..HDR + plen]);
                Rx::Data
            }
            E_RX_EMPTY => Rx::Empty,
            _ => Rx::Gone,
        }
    }

    pub fn state(&self) -> u8 {
        let tx = frame(OP_STATE, &self.handle.to_le_bytes());
        let mut rx = [0u8; HDR + 1];
        let n = mk_ipc_call_timeout(
            self.port as u64,
            tx.as_ptr(),
            tx.len(),
            rx.as_mut_ptr(),
            rx.len(),
            IO_MS,
        );
        if n < (HDR + 1) as i64 || status(&rx) != 0 {
            return 0xff;
        }
        rx[HDR]
    }
}
