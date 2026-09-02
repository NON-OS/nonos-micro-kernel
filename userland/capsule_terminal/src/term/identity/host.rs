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

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use super::choose::choose;
use super::fetch;

const HOST_CAP: usize = 64;

struct Cache(UnsafeCell<[u8; HOST_CAP]>);

/// # Safety
///
/// A capsule is single threaded and the cell is written exactly once, by the
/// first `hostname` call, before `LEN` publishes the length any reader uses.
unsafe impl Sync for Cache {}

static HOST: Cache = Cache(UnsafeCell::new([0u8; HOST_CAP]));
static LEN: AtomicUsize = AtomicUsize::new(0);
static ASKED: AtomicBool = AtomicBool::new(false);

/// The configured hostname, asked for once per capsule.
///
/// `on_enter` runs on the input path and repaints run every frame, so this
/// must never be an IPC round trip after the first: a dead policy server would
/// otherwise cost a timeout per keystroke.
pub fn hostname() -> &'static [u8] {
    if !ASKED.swap(true, Ordering::Relaxed) {
        let mut buf = [0u8; HOST_CAP];
        let n = fetch::hostname(&mut buf).unwrap_or(0);
        let slot: &mut [u8; HOST_CAP] = unsafe { &mut *HOST.0.get() };
        slot[..n].copy_from_slice(&buf[..n]);
        LEN.store(n, Ordering::Release);
    }
    let n = LEN.load(Ordering::Acquire);
    let slot: &'static [u8; HOST_CAP] = unsafe { &*HOST.0.get() };
    choose(&slot[..n])
}
