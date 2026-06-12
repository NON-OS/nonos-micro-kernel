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

pub const CAPACITY: usize = 4096;
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
    pub status: &'static [u8],
    pub path: [u8; 256],
    pub path_len: usize,
    pub prompt: Option<PromptOp>,
    pub shell_port: u32,
}

impl State {
    pub fn new() -> Self {
        let mut path = [0u8; 256];
        path[..PATH.len()].copy_from_slice(PATH);
        State {
            owner_pid: 0,
            buf: [0; CAPACITY],
            len: 0,
            status: b"Ctrl-O open  Ctrl-S save  Ctrl-C copy  Ctrl-V paste",
            path,
            path_len: PATH.len(),
            prompt: None,
            shell_port: 0,
        }
    }

    pub fn backspace(&mut self) -> bool {
        while self.len > 0 {
            self.len -= 1;
            if self.buf[self.len] & 0b1100_0000 != 0b1000_0000 {
                return true;
            }
        }
        false
    }

    pub fn insert(&mut self, bytes: &[u8]) -> bool {
        if self.len + bytes.len() <= CAPACITY {
            self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
            self.len += bytes.len();
            return true;
        }
        false
    }
}
