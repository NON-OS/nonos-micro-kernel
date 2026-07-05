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

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::clients::vfs::{mkdir, rename, rmdir, unlink, write_file};
use nonos_app_skeleton::{EventOutcome, InputEvent, KEY_BACKSPACE, KEY_ENTER, KEY_ESC};

use super::refresh::refresh;
use super::state::{Mode, PromptKind, State};

pub fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    let Mode::Prompt(kind) = state.mode else { return EventOutcome::Idle };
    match event.code {
        KEY_ESC => {
            state.mode = Mode::Browse;
            state.input = String::new();
            state.status = b"cancelled";
        }
        KEY_BACKSPACE => {
            state.input.pop();
        }
        KEY_ENTER => commit(state, kind),
        code => {
            if let Some(ch) = char::from_u32(code) {
                if ch.is_ascii_graphic() && state.input.len() < 64 {
                    state.input.push(ch);
                }
            }
        }
    }
    EventOutcome::Repaint
}

fn commit(state: &mut State, kind: PromptKind) {
    let name = core::mem::take(&mut state.input);
    state.mode = Mode::Browse;
    if let PromptKind::Goto = kind {
        goto(state, &name);
        return;
    }
    if name.is_empty() && !matches!(kind, PromptKind::Delete) {
        state.status = b"empty name";
        return;
    }
    let target = format!("{}{}", state.prefix, name);
    let msg = match run_op(state, kind, &name, &target) {
        Ok(msg) => msg,
        Err(e) => e.as_bytes(),
    };
    super::selection::clear(state);
    refresh(state);
    // Land the cursor on the entry just created or renamed so it is obvious
    // what happened.
    if matches!(kind, PromptKind::NewFile | PromptKind::MkDir | PromptKind::Rename) && !name.is_empty() {
        if let Some(i) = state.entries.iter().position(|e| e.label.trim_end_matches('/') == name) {
            state.cursor = i;
            super::scroll::ensure_visible(state);
        }
    }
    state.status = msg;
}

// Jump to a typed directory: an absolute path is taken as-is, a bare name is
// relative to the current directory; empty goes to the root.
fn goto(state: &mut State, name: &str) {
    let mut dest = if name.is_empty() {
        String::from("/")
    } else if name.starts_with('/') {
        String::from(name)
    } else {
        format!("{}{}", state.prefix, name)
    };
    if !dest.ends_with('/') {
        dest.push('/');
    }
    state.prefix = dest;
    state.cursor = 0;
    state.scroll = 0;
    super::selection::clear(state);
    refresh(state);
}

fn run_op(state: &State, kind: PromptKind, name: &str, target: &str) -> Result<&'static [u8], &'static str> {
    let pid = state.owner_pid;
    match kind {
        PromptKind::NewFile => write_file(pid, target.as_bytes(), b"").map(|_| b"created".as_slice()),
        PromptKind::MkDir => mkdir(pid, target.as_bytes()).map(|_| b"directory created".as_slice()),
        PromptKind::Rename => {
            let old = state.entries.get(state.cursor).ok_or("no selection")?;
            rename(pid, old.full_path.trim_end_matches('/').as_bytes(), target.as_bytes())
                .map(|_| b"renamed".as_slice())
        }
        PromptKind::Delete => {
            if name != "y" {
                return Ok(b"not deleted");
            }
            let targets = super::selection::acting(state);
            if targets.is_empty() {
                return Err("no selection");
            }
            let mut failed = false;
            for (full, is_dir) in &targets {
                // The store keys directories without a trailing slash; a
                // confirmed directory delete removes the whole subtree.
                let result = if *is_dir {
                    rmdir(pid, full.trim_end_matches('/').as_bytes(), true)
                } else {
                    unlink(pid, full.as_bytes())
                };
                if result.is_err() {
                    failed = true;
                }
            }
            Ok(if failed { b"delete: some failed" } else { b"deleted" })
        }
        // Navigation is resolved in `commit` before this point.
        PromptKind::Goto => Ok(b""),
    }
}
