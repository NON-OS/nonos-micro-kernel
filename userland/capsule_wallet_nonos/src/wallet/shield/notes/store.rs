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

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::note::Note;

// The shielded note UTXO set. Insert is O(1) (push); spent-marking is O(log n)
// via a nullifier index (no_std has no hashmap). Balance is reconstructed
// locally from unspent notes; secrets never leave the capsule.
pub struct NoteStore {
    notes: Vec<Note>,
    by_nullifier: BTreeMap<[u8; 32], usize>,
}

impl NoteStore {
    pub fn new() -> Self {
        NoteStore { notes: Vec::new(), by_nullifier: BTreeMap::new() }
    }

    pub fn insert(&mut self, note: Note) {
        let idx = self.notes.len();
        self.by_nullifier.insert(note.nullifier, idx);
        self.notes.push(note);
    }

    // Mark the note with this nullifier spent. Returns false if unknown.
    pub fn mark_spent(&mut self, nullifier: &[u8; 32]) -> bool {
        match self.by_nullifier.get(nullifier) {
            Some(&idx) => match self.notes.get_mut(idx) {
                Some(n) => {
                    n.spent = true;
                    true
                }
                None => false,
            },
            None => false,
        }
    }

    // Unspent balance for one asset, reconstructed locally.
    pub fn balance(&self, asset_id: u32) -> u128 {
        self.notes
            .iter()
            .filter(|n| !n.spent && n.asset_id == asset_id)
            .fold(0u128, |acc, n| acc.saturating_add(n.value))
    }

    pub fn unspent_count(&self) -> usize {
        self.notes.iter().filter(|n| !n.spent).count()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    // Distinct asset ids that currently hold an unspent balance.
    pub fn assets(&self) -> Vec<u32> {
        let mut out: Vec<u32> = Vec::new();
        for n in self.notes.iter().filter(|n| !n.spent) {
            if !out.contains(&n.asset_id) {
                out.push(n.asset_id);
            }
        }
        out
    }
}
