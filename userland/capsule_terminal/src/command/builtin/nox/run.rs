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

use nonos_libc::{mk_ipc_send_to_pid, mk_service_lookup};

use crate::term::state::State;

const CONTROL_MAGIC: u32 = u32::from_le_bytes(*b"NCTL");
const CONTROL_VERSION: u16 = 1;
const OP_FOCUS_SELF: u16 = 1;

// Launch (focus) a desktop app from the shell. Every app capsule is
// already running, so this resolves its service and sends the same
// NCTL focus-self control frame the dock launcher uses, bringing the
// app's window forward.
pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    if args.is_empty() {
        state.scrollback.push_line(
            b"usage: run <app>  (files editor settings calc about procs term)",
        );
        return false;
    }
    let service = resolve_service(args[0]);
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(service.as_ptr(), service.len(), &mut port, &mut pid);
    if rc < 0 || pid == 0 {
        state.scrollback.push_line(b"run: no such app");
        return false;
    }
    let mut frame = [0u8; 8];
    frame[0..4].copy_from_slice(&CONTROL_MAGIC.to_le_bytes());
    frame[4..6].copy_from_slice(&CONTROL_VERSION.to_le_bytes());
    frame[6..8].copy_from_slice(&OP_FOCUS_SELF.to_le_bytes());
    if mk_ipc_send_to_pid(pid, frame.as_ptr(), frame.len()) >= 0 {
        state.scrollback.push_line(b"launched");
        true
    } else {
        state.scrollback.push_line(b"run: launch failed");
        false
    }
}

fn resolve_service(name: &[u8]) -> &[u8] {
    match name {
        b"files" | b"file_manager" => b"app.file_manager",
        b"editor" | b"text_editor" => b"app.text_editor",
        b"settings" => b"app.settings",
        b"calc" | b"calculator" => b"app.calculator",
        b"about" => b"app.about",
        b"procs" | b"processes" | b"process_manager" => b"app.process_manager",
        b"term" => b"app.terminal",
        _ => name,
    }
}
