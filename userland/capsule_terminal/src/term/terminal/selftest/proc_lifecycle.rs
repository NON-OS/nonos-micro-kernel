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

use nonos_libc::{mk_kill, mk_pid_alive, mk_proc_input, mk_wait, mk_yield};

use crate::command::builtin::nox::install::call_installer;

use super::mark;

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
        if let Ok(pid) = call_installer(name, &[]) {
            return Some(pid);
        }
        tries += 1;
        if tries >= SPAWN_TRIES {
            return None;
        }
        mk_yield();
    }
}
