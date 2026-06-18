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
use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use crate::term::state::State;
use crate::term::util::format_u64;

const SERVICE: &[u8] = b"net.dns";
const MAGIC: u32 = 0x4E44_4E53;
const HDR_LEN: usize = 20;
const OP_RESOLVE_A: u16 = 2;
const TIMEOUT_MS: u64 = 2000;

pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    let Some(&host) = args.first() else {
        state.scrollback.push_error(b"usage: nslookup <host>");
        return false;
    };
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(
        SERVICE.as_ptr(),
        SERVICE.len(),
        &mut port as *mut u32,
        &mut pid as *mut u32,
    );
    if rc < 0 || port == 0 {
        state.scrollback.push_error(b"nslookup: dns unavailable");
        return false;
    }
    let mut tx: Vec<u8> = Vec::with_capacity(HDR_LEN + host.len());
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&1u16.to_le_bytes());
    tx.extend_from_slice(&OP_RESOLVE_A.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&1u32.to_le_bytes());
    tx.extend_from_slice(&(host.len() as u32).to_le_bytes());
    tx.extend_from_slice(host);
    let mut rx = [0u8; HDR_LEN + 4];
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n < (HDR_LEN + 4) as i64 || u16::from_le_bytes([rx[8], rx[9]]) != 0 {
        state.scrollback.push_error(b"nslookup: not resolved");
        return false;
    }
    let mut line: Vec<u8> = Vec::new();
    line.extend_from_slice(host);
    line.extend_from_slice(b" -> ");
    for (i, &b) in rx[HDR_LEN..HDR_LEN + 4].iter().enumerate() {
        if i > 0 {
            line.push(b'.');
        }
        let mut buf = [0u8; 24];
        let k = format_u64(b as u64, &mut buf);
        line.extend_from_slice(&buf[..k]);
    }
    state.scrollback.push_line(&line);
    true
}
