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
use alloc::vec::Vec;

use super::opcodes::MAX_PAYLOAD_DWORDS;

/// The host finds each command from the length in the previous header, so a
/// wrong length desynchronises the whole stream rather than failing one
/// command. Lengths are therefore derived here, never supplied.
pub struct Stream {
    words: Vec<u32>,
    poisoned: bool,
}

impl Stream {
    pub fn new() -> Self {
        Self { words: Vec::new(), poisoned: false }
    }

    pub fn push(&mut self, opcode: u8, object: u8, payload: &[u32]) {
        if payload.len() > MAX_PAYLOAD_DWORDS {
            self.poisoned = true;
            return;
        }
        let header = ((payload.len() as u32) << 16) | ((object as u32) << 8) | opcode as u32;
        self.words.push(header);
        self.words.extend_from_slice(payload);
    }

    pub fn finish(self) -> Result<Vec<u8>, &'static str> {
        if self.poisoned {
            return Err("virgl: command payload exceeds the header length field");
        }
        if self.words.is_empty() {
            return Err("virgl: empty command stream");
        }
        let mut out = Vec::with_capacity(self.words.len() * 4);
        for word in &self.words {
            out.extend_from_slice(&word.to_le_bytes());
        }
        Ok(out)
    }
}

impl Default for Stream {
    fn default() -> Self {
        Self::new()
    }
}
