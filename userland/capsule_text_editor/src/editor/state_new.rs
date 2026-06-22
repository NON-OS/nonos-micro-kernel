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

use super::state::{State, PATH};

impl State {
    pub fn new() -> Self {
        let mut path = [0u8; 256];
        path[..PATH.len()].copy_from_slice(PATH);
        State {
            owner_pid: 0,
            buf: [0; super::state::CAPACITY],
            len: 0,
            scroll_line: 0,
            visible_rows: 1,
            wrap_cols: 80,
            status: b"Ctrl-O open  Ctrl-S save  Ctrl-C copy  Ctrl-V paste",
            path,
            path_len: PATH.len(),
            prompt: None,
            shell_port: 0,
        }
    }
}
