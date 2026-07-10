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

//! A rasterized-glyph cache. Outlining a glyph with ab_glyph is expensive, so a
//! full screen of text every keystroke would be slow. Each (face, glyph, size)
//! is rasterized once into a coverage bitmap and reused; the colour is applied
//! at blit time, so the same glyph serves every colour. Capsules are single
//! threaded, so the lock never actually contends.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use spin::Mutex;

/// A glyph's coverage bitmap and its offset from the pen origin.
pub(super) struct Raster {
    pub min_x: i32,
    pub min_y: i32,
    pub w: u32,
    pub h: u32,
    pub cov: Vec<u8>,
}

// (mono face, glyph id, px size bits).
type Key = (bool, u16, u32);

/// Guard against an unbounded cache if a program renders at many sizes; the
/// working set for a UI is far smaller than this.
const MAX_ENTRIES: usize = 8192;

static CACHE: Mutex<Option<BTreeMap<Key, Option<Raster>>>> = Mutex::new(None);

/// Fetch the cached raster for `key`, rasterizing on a miss, then run `blit`.
/// A glyph with no outline (a space) caches as None so it is not retried.
pub(super) fn with_raster<R>(
    key: Key,
    rasterize: impl FnOnce() -> Option<Raster>,
    blit: impl FnOnce(&Raster) -> R,
) -> Option<R> {
    let mut guard = CACHE.lock();
    let map = guard.get_or_insert_with(BTreeMap::new);
    if map.len() >= MAX_ENTRIES && !map.contains_key(&key) {
        map.clear();
    }
    let entry = map.entry(key).or_insert_with(rasterize);
    entry.as_ref().map(blit)
}
