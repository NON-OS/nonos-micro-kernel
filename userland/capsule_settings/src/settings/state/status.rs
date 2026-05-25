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

const MAX_MSG: usize = 96;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StatusKind {
    Idle,
    Ok,
    Error,
}

#[derive(Clone, Copy)]
pub struct Status {
    pub kind: StatusKind,
    pub bytes: [u8; MAX_MSG],
    pub len: usize,
}

impl Status {
    pub const fn idle() -> Self {
        Self { kind: StatusKind::Idle, bytes: [0u8; MAX_MSG], len: 0 }
    }

    pub fn set(&mut self, kind: StatusKind, text: &[u8]) {
        self.kind = kind;
        let n = core::cmp::min(text.len(), MAX_MSG);
        self.bytes[..n].copy_from_slice(&text[..n]);
        self.len = n;
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}
