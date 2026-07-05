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

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::read_file;
use nonos_app_skeleton::{EventOutcome, InputEvent, KEY_BACKSPACE, KEY_DOWN, KEY_ESC, KEY_LEFT, KEY_UP};

use super::preview_hex::hex_lines;
use super::preview_paint::VISIBLE_LINES;
use super::preview_text::text_lines;
use super::state::{Mode, State};

// Cap the bytes pulled for a preview so opening a huge file stays bounded; the
// header reports when a file was longer than what is shown.
const MAX_PREVIEW_BYTES: u32 = 256 * 1024;

// An opened file rendered as scrollable lines. Text files show their content;
// binary files show a hexdump. `scroll` is the index of the first visible line.
pub struct Preview {
    pub path: String,
    pub lines: Vec<String>,
    pub byte_len: usize,
    pub binary: bool,
    pub truncated: bool,
    pub scroll: usize,
}

// Read `path` through the vfs and switch the view to the preview of its
// content. A failed read leaves browse mode with an error status.
pub fn open_preview(state: &mut State, path: String) {
    match read_file(state.owner_pid, path.as_bytes(), MAX_PREVIEW_BYTES) {
        Ok(bytes) => {
            let binary = is_binary(&bytes);
            let lines = if binary { hex_lines(&bytes) } else { text_lines(&bytes) };
            state.preview = Some(Preview {
                path,
                byte_len: bytes.len(),
                binary,
                truncated: bytes.len() >= MAX_PREVIEW_BYTES as usize,
                lines,
                scroll: 0,
            });
            state.mode = Mode::Preview;
            state.status = b"esc back  up/down scroll";
        }
        Err(_) => {
            state.preview = None;
            state.status = b"read failed";
        }
    }
}

// Keys while a file is open: scroll a line at a time (arrows or vim j/k), or
// leave the preview back to the listing.
pub fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    let Some(pv) = state.preview.as_mut() else {
        state.mode = Mode::Browse;
        return EventOutcome::Repaint;
    };
    let max_scroll = pv.lines.len().saturating_sub(VISIBLE_LINES);
    match event.code {
        KEY_ESC | KEY_BACKSPACE | KEY_LEFT => {
            state.mode = Mode::Browse;
            state.preview = None;
            state.status = b"click or Enter to open";
        }
        KEY_DOWN => pv.scroll = (pv.scroll + 1).min(max_scroll),
        KEY_UP => pv.scroll = pv.scroll.saturating_sub(1),
        code if code == b'j' as u32 => pv.scroll = (pv.scroll + 1).min(max_scroll),
        code if code == b'k' as u32 => pv.scroll = pv.scroll.saturating_sub(1),
        code if code == b' ' as u32 => pv.scroll = (pv.scroll + VISIBLE_LINES).min(max_scroll),
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}

// A file counts as binary when a leading sample carries a NUL or too many bytes
// outside the printable-and-whitespace range, so it renders as a hexdump rather
// than garbled text.
fn is_binary(bytes: &[u8]) -> bool {
    let sample = &bytes[..bytes.len().min(1024)];
    if sample.is_empty() {
        return false;
    }
    let mut suspicious = 0usize;
    for &b in sample {
        if b == 0 {
            return true;
        }
        let printable = (0x20..=0x7e).contains(&b) || matches!(b, b'\n' | b'\r' | b'\t');
        if !printable {
            suspicious += 1;
        }
    }
    suspicious * 100 / sample.len() > 30
}
