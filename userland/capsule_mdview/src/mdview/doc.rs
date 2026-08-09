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

use super::layout::{parse, Block};
use super::load::read_doc;

// The vfs service can still be registering when the first frame lands, so one
// attempt is too few; retrying every frame would pour sync IPC calls into a
// path whose replies already time out at 16ms. Three bounded tries settle it.
const MAX_ATTEMPTS: u8 = 3;

pub struct Doc {
    pub blocks: Vec<Block>,
    pub error: Option<&'static str>,
    attempts: u8,
}

impl Doc {
    pub fn new() -> Self {
        Doc {
            blocks: Vec::new(),
            error: None,
            attempts: 0,
        }
    }

    pub fn ensure(&mut self) -> bool {
        if !self.blocks.is_empty() || self.attempts >= MAX_ATTEMPTS {
            return false;
        }
        self.attempts += 1;
        match read_doc() {
            Ok(text) => {
                self.blocks = parse(&text);
                self.error = None;
                true
            }
            Err(message) => {
                self.error = Some(message);
                false
            }
        }
    }
}
