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

//! Home screen state. The screen is repainted from scratch every frame and the
//! dispatcher hands it no storage of its own, so the selected nav row and the
//! width the last paint used live in process-wide atomics, the same shape
//! `theme` uses for the active palette.

use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

pub(super) const NAV_LABELS: [&str; 6] = [
    "Home",
    "Recent",
    "Starred",
    "Shared with me",
    "Templates",
    "Trash",
];

pub(super) const NAV_LIVE: [bool; 6] = [true, true, false, false, false, false];

static NAV: AtomicUsize = AtomicUsize::new(0);
static PAINTED_W: AtomicU32 = AtomicU32::new(0);
static PAINTED_H: AtomicU32 = AtomicU32::new(0);
static VIEW_X: AtomicU32 = AtomicU32::new(0);
static VIEW_Y: AtomicU32 = AtomicU32::new(0);
static VIEW_W: AtomicU32 = AtomicU32::new(0);
static VIEW_H: AtomicU32 = AtomicU32::new(0);

pub(super) struct HomeState {
    pub nav: usize,
}

impl HomeState {
    pub(super) fn load() -> Self {
        Self { nav: NAV.load(Ordering::Relaxed) % NAV_LABELS.len() }
    }

    pub(super) fn select(nav: usize) -> bool {
        NAV_LIVE.get(nav).copied().unwrap_or(false) && NAV.swap(nav, Ordering::Relaxed) != nav
    }

    pub(super) fn note_size(w: u32, h: u32) {
        PAINTED_W.store(w, Ordering::Relaxed);
        PAINTED_H.store(h, Ordering::Relaxed);
    }

    pub(super) fn painted_width() -> u32 {
        PAINTED_W.load(Ordering::Relaxed)
    }

    pub(super) fn painted_height() -> u32 {
        PAINTED_H.load(Ordering::Relaxed)
    }

    pub(super) fn note_view_all(rect: (u32, u32, u32, u32)) {
        VIEW_X.store(rect.0, Ordering::Relaxed);
        VIEW_Y.store(rect.1, Ordering::Relaxed);
        VIEW_W.store(rect.2, Ordering::Relaxed);
        VIEW_H.store(rect.3, Ordering::Relaxed);
    }

    pub(super) fn view_all_hit(mx: i32, my: i32) -> bool {
        let (x, y) = (VIEW_X.load(Ordering::Relaxed), VIEW_Y.load(Ordering::Relaxed));
        let (w, h) = (VIEW_W.load(Ordering::Relaxed), VIEW_H.load(Ordering::Relaxed));
        w != 0 && h != 0 && mx >= x as i32 && my >= y as i32
            && mx < (x + w) as i32 && my < (y + h) as i32
    }
}
