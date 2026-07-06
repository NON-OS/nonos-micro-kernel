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

use super::preview_hex::hex_lines;
use super::preview_is_binary::is_binary;
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
