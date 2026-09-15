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

extern crate alloc;

use alloc::{string::String, vec::Vec};

// Bounded because the stack holds only in-memory inverses; it is not persisted
// across a restart and is not a substitute for a trash namespace.
const UNDO_DEPTH: usize = 16;

// The already-computed inverse of a completed operation, so undo never has to
// re-derive it from a forward record.
pub enum Op {
    Rename { from: String, to: String },
    Rmdir { path: String },
    Unlink { path: String },
    Chmod { path: String, writable: bool },
}

#[derive(Default)]
pub struct UndoStack {
    ops: Vec<Op>,
}

impl UndoStack {
    pub fn push(&mut self, op: Op) {
        self.ops.push(op);
        if self.ops.len() > UNDO_DEPTH {
            self.ops.remove(0);
        }
    }

    pub fn pop(&mut self) -> Option<Op> {
        self.ops.pop()
    }

    // A destructive delete has no inverse: the vfs keeps no trash and no
    // content journal, so the stack is emptied rather than offering an undo
    // that would silently fail.
    pub fn clear(&mut self) {
        self.ops.clear();
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }
}
