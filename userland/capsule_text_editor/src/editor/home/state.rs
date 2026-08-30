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

static NAV: AtomicUsize = AtomicUsize::new(0);
static PAINTED_W: AtomicU32 = AtomicU32::new(0);

pub(super) struct HomeState {
    pub nav: usize,
}

impl HomeState {
    pub(super) fn load() -> Self {
        Self { nav: NAV.load(Ordering::Relaxed) % NAV_LABELS.len() }
    }

    pub(super) fn select(nav: usize) -> bool {
        nav < NAV_LABELS.len() && NAV.swap(nav, Ordering::Relaxed) != nav
    }

    pub(super) fn note_width(w: u32) {
        PAINTED_W.store(w, Ordering::Relaxed);
    }

    pub(super) fn painted_width() -> u32 {
        PAINTED_W.load(Ordering::Relaxed)
    }
}
