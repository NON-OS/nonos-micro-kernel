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

use crate::doc::block::Block;

#[derive(Clone, PartialEq, Debug)]
pub struct Doc {
    pub blocks: Vec<Block>,
}

impl Doc {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn covered(&self) -> bool {
        self.blocks.iter().all(|b| b.covered())
    }
}

impl Default for Doc {
    fn default() -> Self {
        Self::new()
    }
}
