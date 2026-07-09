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

use nonos_app_skeleton::discover::lookup_service;
use nonos_libc::{mk_ipc_call, mk_kill, mk_pid_alive, mk_proc_input, mk_wait, mk_yield};

use super::mark;

const OP_LOAD_BY_NAME: u16 = 4;
const SEQ: u32 = 1;
const REQUESTED_CAPS: u64 = u64::MAX;
const SIGTERM: u64 = 15;
const SIGTERM_STATUS: i64 = 143;
const HELLO_EXIT: i64 = 2;
const WAIT_MS: u64 = 30_000;
const SPAWN_TRIES: u32 = 100_000;

pub fn run() {
    wait_ok();
    kill_ok();
    stdin_ok();
}

fn wait_ok() {
    match spawn(b"hello") {
        Some(child) => mark(b"wait", mk_wait(child as u64, WAIT_MS) == HELLO_EXIT),
        None => mark(b"wait", false),
    }
}

fn kill_ok() {
    let Some(child) = spawn(b"hello") else { return mark(b"kill", false) };
    let killed = mk_kill(child as u64, SIGTERM) == 0;
    let reaped = mk_wait(child as u64, WAIT_MS) == SIGTERM_STATUS;
    mark(b"kill", killed && reaped && !mk_pid_alive(child));
}

fn stdin_ok() {
    let Some(child) = spawn(b"hello") else { return mark(b"stdin", false) };
    let buf = [0u8; 8];
    let accepted = mk_proc_input(child as u64, buf.as_ptr(), buf.len() as u64) == buf.len() as i64;
    mark(b"stdin", accepted);
    let _ = mk_kill(child as u64, SIGTERM);
}

fn spawn(name: &[u8]) -> Option<u32> {
    let mut tries = 0u32;
    loop {
        if let Some(pid) = load_by_name(name) {
            return Some(pid);
        }
        tries += 1;
        if tries >= SPAWN_TRIES {
            return None;
        }
        mk_yield();
    }
}

fn load_by_name(name: &[u8]) -> Option<u32> {
    let port = lookup_service(b"installer").map(|p| p.port)?;
    let payload = pack(name);
    let mut rx = [0u8; 32];
    let rc = mk_ipc_call(port as u64, payload.as_ptr(), payload.len(), rx.as_mut_ptr(), rx.len());
    if rc < 12 {
        return None;
    }
    if i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]) != 0 {
        return None;
    }
    Some(u32::from_le_bytes([rx[8], rx[9], rx[10], rx[11]]))
}

fn pack(name: &[u8]) -> Vec<u8> {
    let mut p = Vec::with_capacity(17 + name.len());
    p.extend_from_slice(&SEQ.to_le_bytes());
    p.extend_from_slice(&OP_LOAD_BY_NAME.to_le_bytes());
    p.extend_from_slice(&[0u8, 0u8]);
    p.extend_from_slice(&REQUESTED_CAPS.to_le_bytes());
    p.push(name.len() as u8);
    p.extend_from_slice(name);
    p
}
