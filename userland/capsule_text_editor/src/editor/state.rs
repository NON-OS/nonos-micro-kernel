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

pub const CAPACITY: usize = 16384;
pub const PATH: &[u8] = b"/notes.txt";

#[derive(Clone, Copy, PartialEq)]
pub enum PromptOp {
    Open,
    Save,
}

pub struct State {
    pub owner_pid: u32,
    pub buf: [u8; CAPACITY],
    pub len: usize,
    pub scroll_line: u32,
    pub visible_rows: u32,
    pub wrap_cols: u32,
    pub status: &'static [u8],
    pub path: [u8; 256],
    pub path_len: usize,
    pub prompt: Option<PromptOp>,
    pub shell_port: u32,
}
