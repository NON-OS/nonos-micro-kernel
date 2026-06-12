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

use nonos_libc::mk_yield;

// Give a virtio-gpu driver time to announce its gfx service before
// falling back to the GOP framebuffer path, so a machine that has
// virtio-gpu always uses it and only real hardware / hypervisors
// without it take the GOP route.
const VIRTIO_ATTEMPTS_BEFORE_GOP: u32 = 6;

pub fn wait_for_setup() -> crate::state::Context {
    let mut attempt: u32 = 0;
    loop {
        if let Ok(ctx) = crate::setup::run_virtio() {
            return ctx;
        }
        if attempt >= VIRTIO_ATTEMPTS_BEFORE_GOP {
            if let Ok(ctx) = crate::setup::run_gop() {
                return ctx;
            }
        }
        attempt = attempt.saturating_add(1);
        for _ in 0..64 {
            mk_yield();
        }
    }
}
