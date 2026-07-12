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

//! Local shielded note store (the UTXO set), view-tag scanner, and incremental
//! merkle-path cache. Scalable from day one: O(1) insert, index-backed
//! spent-marking, O(1)-per-note view-tag sync, recent-roots path window.

mod merkle_cache;
mod note;
mod scanner;
mod store;

pub use merkle_cache::{CachedPath, MerkleCache};
pub use note::Note;
pub use scanner::{ChainNote, NoteScanner, StubScanner};
pub use store::NoteStore;
