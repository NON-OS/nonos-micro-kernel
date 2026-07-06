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

use nonos_toolkit::font::ttf::FontRef;
use spin::Mutex;

// Loaded page fonts keyed by family hash. A global rather than a threaded
// parameter because glyph measurement happens deep inside layout, which never
// sees the page state; the capsule is single threaded so the lock is only a
// formality. Cleared on navigation with the rest of the page.
static FONTS: Mutex<Vec<(u32, Vec<u8>)>> = Mutex::new(Vec::new());

const MAX_FONTS: usize = 8;
const MAX_FONT_BYTES: usize = 2 * 1024 * 1024;

// Validate and install a fetched face under its family key. Returns true when
// the face parsed and text using it should relayout.
pub fn install(key: u32, bytes: Vec<u8>) -> bool {
    if key == 0 || bytes.len() > MAX_FONT_BYTES || FontRef::try_from_slice(&bytes).is_err() {
        return false;
    }
    let mut fonts = FONTS.lock();
    if fonts.len() >= MAX_FONTS || fonts.iter().any(|(k, _)| *k == key) {
        return false;
    }
    fonts.push((key, bytes));
    true
}

pub fn clear() {
    FONTS.lock().clear();
}

// Run `f` with the parsed face for `key`. The face borrows the registry entry
// for the duration of the call.
pub fn with_face<R>(key: u32, f: impl FnOnce(&FontRef) -> R) -> Option<R> {
    if key == 0 {
        return None;
    }
    let fonts = FONTS.lock();
    let bytes = fonts.iter().find(|(k, _)| *k == key).map(|(_, b)| b)?;
    let face = FontRef::try_from_slice(bytes).ok()?;
    Some(f(&face))
}
