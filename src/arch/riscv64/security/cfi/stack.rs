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

use super::error::CfiError;

pub const SHADOW_STACK_CAPACITY: usize = 256;

pub struct ShadowStack {
    entries: [usize; SHADOW_STACK_CAPACITY],
    depth: usize,
    enabled: bool,
}

impl ShadowStack {
    pub const fn new() -> Self {
        Self { entries: [0; SHADOW_STACK_CAPACITY], depth: 0, enabled: false }
    }

    pub fn enable(&mut self) {
        self.depth = 0;
        self.enabled = true;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn push(&mut self, ra: usize) -> Result<(), CfiError> {
        if !self.enabled {
            return Err(CfiError::Disabled);
        }
        if self.depth == SHADOW_STACK_CAPACITY {
            return Err(CfiError::Overflow);
        }
        self.entries[self.depth] = ra;
        self.depth += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<usize, CfiError> {
        if !self.enabled {
            return Err(CfiError::Disabled);
        }
        if self.depth == 0 {
            return Err(CfiError::Underflow);
        }
        self.depth -= 1;
        let ra = self.entries[self.depth];
        self.entries[self.depth] = 0;
        Ok(ra)
    }
}
