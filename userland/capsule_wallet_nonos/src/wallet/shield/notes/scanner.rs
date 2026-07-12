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

use super::note::Note;
use crate::wallet::pool::Seam;

// One on-chain shielded output from the chain feed. `view_tag` lets the wallet
// skip notes that are not ours in O(1), before any trial-decrypt.
#[derive(Clone, Copy)]
pub struct ChainNote {
    pub view_tag: u8,
    pub ciphertext: [u8; 96],
    pub leaf_index: u64,
}

// View-tag scanner: sync is O(1) per on-chain note, not a trial-decrypt of the
// whole tree. `our_view_tag` is derived from the wallet's viewing key. The Stub
// reports NotWired (no chain feed / keys yet); a live impl decrypts matches.
pub trait NoteScanner {
    // The wallet's own view tag; a chain note matches only if tags are equal.
    fn our_view_tag(&self) -> u8;

    // O(1) prefilter: does this on-chain note's tag match ours?
    fn matches(&self, cn: &ChainNote) -> bool {
        cn.view_tag == self.our_view_tag()
    }

    // Decrypt a matched note into a local Note. Only called after `matches`.
    fn recover(&self, cn: &ChainNote) -> Seam<Note>;
}

// No viewing key / chain feed connected yet.
pub struct StubScanner;

impl NoteScanner for StubScanner {
    fn our_view_tag(&self) -> u8 {
        0
    }
    fn recover(&self, _cn: &ChainNote) -> Seam<Note> {
        Seam::NotWired
    }
}
