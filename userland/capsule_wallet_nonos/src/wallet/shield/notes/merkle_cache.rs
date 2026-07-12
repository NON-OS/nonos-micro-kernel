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

const RECENT_ROOTS_WINDOW: usize = 16;

// A cached authentication path for one note under a specific root.
#[derive(Clone)]
pub struct CachedPath {
    pub root: [u8; 32],
    pub siblings: Vec<[u8; 32]>,
}

// Incremental merkle-path cache keyed by leaf_index. The on-chain root moves, so
// a cached path is only usable while its root is still inside the recent-roots
// window; do not assume a single fixed root.
pub struct MerkleCache {
    paths: BTreeMap<u64, CachedPath>,
    recent_roots: Vec<[u8; 32]>,
}

impl MerkleCache {
    pub fn new() -> Self {
        MerkleCache { paths: BTreeMap::new(), recent_roots: Vec::new() }
    }

    pub fn put(&mut self, leaf_index: u64, path: CachedPath) {
        self.paths.insert(leaf_index, path);
    }

    pub fn get(&self, leaf_index: u64) -> Option<&CachedPath> {
        self.paths.get(&leaf_index)
    }

    // Refresh the window of on-chain roots a path may be proven against.
    pub fn set_recent_roots(&mut self, mut roots: Vec<[u8; 32]>) {
        if roots.len() > RECENT_ROOTS_WINDOW {
            let drop = roots.len() - RECENT_ROOTS_WINDOW;
            roots.drain(0..drop);
        }
        self.recent_roots = roots;
    }

    // A path is usable only if its root is still in the recent window.
    pub fn is_usable(&self, leaf_index: u64) -> bool {
        match self.paths.get(&leaf_index) {
            Some(p) => self.recent_roots.iter().any(|r| *r == p.root),
            None => false,
        }
    }
}
