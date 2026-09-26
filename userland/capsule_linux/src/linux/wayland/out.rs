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


//! Building an event for the client.

use alloc::vec::Vec;

use super::wire::HEADER;

pub struct Event {
    body: Vec<u8>,
}

impl Event {
    pub fn new(object: u32, opcode: u16) -> Event {
        let mut body = Vec::with_capacity(HEADER + 16);
        body.extend_from_slice(&object.to_le_bytes());
        body.extend_from_slice(&opcode.to_le_bytes());
        body.extend_from_slice(&0u16.to_le_bytes());
        Event { body }
    }

    pub fn u32(mut self, value: u32) -> Event {
        self.body.extend_from_slice(&value.to_le_bytes());
        self
    }

    /// A string is its length including the terminator, the bytes, the
    /// terminator, then zeroes up to the next word.
    pub fn string(mut self, text: &[u8]) -> Event {
        self.body.extend_from_slice(&(text.len() as u32 + 1).to_le_bytes());
        self.body.extend_from_slice(text);
        self.body.push(0);
        while !self.body.len().is_multiple_of(4) {
            self.body.push(0);
        }
        self
    }

    pub fn send(mut self, to_client: &mut Vec<u8>) {
        let size = self.body.len() as u16;
        self.body[6..8].copy_from_slice(&size.to_le_bytes());
        to_client.extend_from_slice(&self.body);
    }
}
