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

//! Screen-local Settings state. The selected nav row and the four switch rows
//! live process-wide in atomics, the same shape `theme` uses for its active
//! index, so paint and event agree without a field on `Editor`.

use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

pub(super) struct SettingsState {
    pub nav: usize,
    pub switches: [bool; 4],
}

static NAV: AtomicUsize = AtomicUsize::new(0);
static SWITCHES: AtomicU32 = AtomicU32::new(0b0111);

pub(super) fn state() -> SettingsState {
    let bits = SWITCHES.load(Ordering::Relaxed);
    let mut switches = [false; 4];
    for (i, s) in switches.iter_mut().enumerate() {
        *s = bits & (1 << i) != 0;
    }
    SettingsState { nav: NAV.load(Ordering::Relaxed), switches }
}

pub(super) fn select_nav(index: usize) -> bool {
    NAV.swap(index, Ordering::Relaxed) != index
}

pub(super) fn flip_switch(index: usize) {
    SWITCHES.fetch_xor(1 << index, Ordering::Relaxed);
}

static CONTENT_W: AtomicU32 = AtomicU32::new(0);

pub(super) fn latch_width(width: u32) {
    CONTENT_W.store(width, Ordering::Relaxed);
}

pub(super) fn width() -> u32 {
    CONTENT_W.load(Ordering::Relaxed)
}
