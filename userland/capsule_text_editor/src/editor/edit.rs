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

//! The single edit path. Every mutation replaces a byte range with new bytes
//! through `apply_edit`, which records a reversible operation. Undo and redo
//! replay these, so the buffer, caret, and history never drift apart. All
//! splicing is bounds-checked, so a bad offset can only no-op, never corrupt.

use alloc::vec::Vec;

use super::state::{State, CAPACITY};

/// A reversible edit: at `at`, `inserted_len` bytes replaced `deleted`.
pub struct EditOp {
    pub at: usize,
    pub deleted: Vec<u8>,
    pub inserted_len: usize,
}

const MAX_UNDO: usize = 400;

impl State {
    /// Replace `buf[at..at+del]` with `ins`, recording an undo step and clearing
    /// the redo stack. Returns false if it would overflow the buffer.
    pub fn apply_edit(&mut self, at: usize, del: usize, ins: &[u8]) -> bool {
        let Some(removed) = self.splice(at, del, ins) else {
            return false;
        };
        self.push_undo(at, removed, ins.len());
        self.redo.clear();
        true
    }

    pub fn undo(&mut self) -> bool {
        let Some(op) = self.undo.pop() else {
            return false;
        };
        match self.apply_op(&op) {
            Some(reverse) => {
                self.redo.push(reverse);
                true
            }
            None => {
                self.undo.push(op);
                false
            }
        }
    }

    pub fn redo(&mut self) -> bool {
        let Some(op) = self.redo.pop() else {
            return false;
        };
        match self.apply_op(&op) {
            Some(reverse) => {
                self.undo.push(reverse);
                true
            }
            None => {
                self.redo.push(op);
                false
            }
        }
    }

    // Splice bytes without touching history; returns the removed bytes.
    fn splice(&mut self, at: usize, del: usize, ins: &[u8]) -> Option<Vec<u8>> {
        let at = at.min(self.len);
        let del = del.min(self.len - at);
        if self.len - del + ins.len() > CAPACITY {
            return None;
        }
        let removed = self.buf[at..at + del].to_vec();
        self.buf.copy_within(at + del..self.len, at + ins.len());
        self.buf[at..at + ins.len()].copy_from_slice(ins);
        self.len = self.len - del + ins.len();
        self.caret = at + ins.len();
        self.reflow();
        Some(removed)
    }

    // Apply one op and return its inverse, for undo/redo replay.
    fn apply_op(&mut self, op: &EditOp) -> Option<EditOp> {
        let removed = self.splice(op.at, op.inserted_len, &op.deleted)?;
        self.caret = op.at + op.deleted.len();
        Some(EditOp { at: op.at, deleted: removed, inserted_len: op.deleted.len() })
    }

    // Push an undo step, coalescing a run of single-character typing so one
    // Ctrl-Z removes a word rather than a letter.
    fn push_undo(&mut self, at: usize, removed: Vec<u8>, inserted_len: usize) {
        if removed.is_empty() && inserted_len == 1 {
            if let Some(last) = self.undo.last_mut() {
                if last.deleted.is_empty() && last.at + last.inserted_len == at {
                    last.inserted_len += 1;
                    return;
                }
            }
        }
        self.undo.push(EditOp { at, deleted: removed, inserted_len });
        if self.undo.len() > MAX_UNDO {
            self.undo.remove(0);
        }
    }
}
