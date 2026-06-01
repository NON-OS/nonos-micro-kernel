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

use nonos_app_skeleton::{clipboard_copy, clipboard_paste, clients::vfs, EventOutcome, InputEvent, KEY_BACKSPACE, KEY_ENTER, KEY_ESC, MOD_CTRL};

use super::state::{State, PATH};

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    if event.flags & MOD_CTRL != 0 {
        return on_ctrl(state, event.code);
    }
    let changed = match event.code {
        KEY_ESC => return EventOutcome::Close,
        KEY_BACKSPACE => state.backspace(),
        KEY_ENTER => state.insert(b"\n"),
        code if (0x20..=0x10FFFF).contains(&code) => {
            let mut scratch = [0u8; 4];
            match char::from_u32(code).map(|ch| ch.encode_utf8(&mut scratch).as_bytes()) {
                Some(bytes) => state.insert(bytes),
                None => false,
            }
        }
        _ => false,
    };
    if changed {
        state.status = b"edited /notes.txt";
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}

fn on_ctrl(state: &mut State, code: u32) -> EventOutcome {
    match code {
        c if matches!(c, 0x43 | 0x63) => {
            state.status = if clipboard_copy(&state.buf[..state.len]).is_ok() { b"copied /notes.txt" } else { b"clipboard unavailable" };
            EventOutcome::Repaint
        }
        c if matches!(c, 0x4F | 0x6F) => {
            match vfs::read_file(state.owner_pid, PATH, super::state::CAPACITY as u32) {
                Ok(bytes) if core::str::from_utf8(&bytes).is_ok() && bytes.len() <= super::state::CAPACITY => {
                    state.buf[..bytes.len()].copy_from_slice(&bytes);
                    state.len = bytes.len();
                    state.status = b"opened /notes.txt";
                }
                Ok(_) => state.status = b"file is not valid utf-8",
                Err(_) => state.status = b"open failed",
            }
            EventOutcome::Repaint
        }
        c if matches!(c, 0x53 | 0x73) => {
            state.status = if vfs::write_file(state.owner_pid, PATH, &state.buf[..state.len]).is_ok() { b"saved /notes.txt" } else { b"save failed" };
            EventOutcome::Repaint
        }
        c if matches!(c, 0x56 | 0x76) => {
            let mut scratch = [0u8; 512];
            match clipboard_paste(&mut scratch) {
                Ok(n) if core::str::from_utf8(&scratch[..n]).is_ok() && state.insert(&scratch[..n]) => {
                    state.status = b"pasted into /notes.txt";
                    EventOutcome::Repaint
                }
                Ok(_) => {
                    state.status = b"paste rejected";
                    EventOutcome::Repaint
                }
                Err(_) => {
                    state.status = b"clipboard unavailable";
                    EventOutcome::Repaint
                }
            }
        }
        _ => EventOutcome::Idle,
    }
}
