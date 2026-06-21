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

use super::types::{Block, Status};
use crate::term::state::State;

const MAX_BLOCKS: usize = 256;

impl State {
    pub fn open_block(&mut self, ts: [u8; 8]) {
        let start_abs = self.scrollback.grid.current_abs_line();
        self.blocks.push(Block { start_abs, ts, status: Status::Running });
        if self.blocks.len() > MAX_BLOCKS {
            self.blocks.remove(0);
        }
    }

    pub fn close_block(&mut self, ok: bool) {
        if let Some(b) = self.blocks.last_mut() {
            b.status = if ok { Status::Ok } else { Status::Err };
        }
    }

    pub fn evict_blocks(&mut self) {
        let base = self.scrollback.grid.abs_base();
        let keep = self.blocks.iter().position(|b| b.start_abs >= base).unwrap_or(self.blocks.len());
        if keep > 1 {
            self.blocks.drain(..keep - 1);
        }
    }

    pub fn block_at(&self, abs: u64) -> Option<&Block> {
        self.blocks.iter().rev().find(|b| b.start_abs <= abs)
    }
}
