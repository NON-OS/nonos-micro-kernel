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

use super::uptime::uptime_ms;
use crate::sys::sync::IrqMutex;
use core::sync::atomic::{AtomicU64, Ordering};

const MAX_CALLBACKS: usize = 16;

pub type TimerCallback = fn();

static CALLBACKS: IrqMutex<[Option<(TimerCallback, u64, u64)>; MAX_CALLBACKS]> =
    IrqMutex::new([None; MAX_CALLBACKS]);
pub static CALLBACK_COUNT: AtomicU64 = AtomicU64::new(0);

pub fn register_callback(callback: TimerCallback, interval_ms: u64) -> Option<usize> {
    let mut callbacks = CALLBACKS.lock();
    let count = CALLBACK_COUNT.load(Ordering::Acquire) as usize;
    if count >= MAX_CALLBACKS {
        return None;
    }

    let next_trigger = uptime_ms() + interval_ms;
    callbacks[count] = Some((callback, interval_ms, next_trigger));
    CALLBACK_COUNT.fetch_add(1, Ordering::Release);
    Some(count)
}

pub fn unregister_callback(id: usize) {
    if id < MAX_CALLBACKS {
        let mut callbacks = CALLBACKS.lock();
        callbacks[id] = None;
    }
}

pub fn process_callbacks() {
    let now = uptime_ms();
    let count = CALLBACK_COUNT.load(Ordering::Acquire) as usize;
    let mut callbacks = CALLBACKS.lock();

    for i in 0..count {
        if let Some((callback, interval, next_trigger)) = callbacks[i] {
            if now >= next_trigger {
                drop(callbacks);
                callback();
                callbacks = CALLBACKS.lock();
                if i < MAX_CALLBACKS {
                    callbacks[i] = Some((callback, interval, now + interval));
                }
            }
        }
    }
}
