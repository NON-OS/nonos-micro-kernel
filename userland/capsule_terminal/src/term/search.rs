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

//! Searching backwards through what has been run.
//!
//! Recall by prefix answers "what did I run that started like this". Search
//! answers "what did I run that mentioned this", which is the question a
//! reader usually has, because what they remember is a word from the middle
//! of a command rather than how it began.

use alloc::vec::Vec;

/// An in-progress reverse search.
pub struct Search {
    /// What the reader has typed to search for.
    pub needle: Vec<u8>,
    /// Where the current match sits, so stepping asks for one older.
    pub at: usize,
    /// The line as it was before the search began, restored on cancel.
    pub saved: Vec<u8>,
}

impl Search {
    pub fn new(saved: Vec<u8>, count: usize) -> Self {
        Self { needle: Vec::new(), at: count, saved }
    }
}
