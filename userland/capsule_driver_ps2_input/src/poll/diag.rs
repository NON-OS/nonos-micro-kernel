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
use core::sync::atomic::{AtomicBool, Ordering};

use nonos_libc::{mk_input_event_post, InputEvent};

// Diagnostic sentinels the kernel counts and paints as on-screen bars; they
// never route as input. 0xFE00 fires once when a port read first succeeds,
// proving the PIO grant is live and the kernel can reach 0x60/0x64 for this
// capsule. 0xFE03 fires per byte the i8042 actually hands us. Read together:
// no PIO-OK means the grant is broken; PIO-OK with no bytes while typing means
// the controller is silent (kbd port disabled or scanning off), not the grant.
const KIND_PIO_OK: u16 = 0xFE00;
const KIND_I8042_BYTE: u16 = 0xFE03;

static PIO_OK_SIGNALED: AtomicBool = AtomicBool::new(false);

fn post(kind: u16) {
    let ev =
        InputEvent { kind, flags: 0, code: 0, x: 0, y: 0, delta_x: 0, delta_y: 0, timestamp_ns: 0 };
    let _ = mk_input_event_post(&ev);
}

/// Report the first successful port read. Idempotent: only the first call posts.
pub fn signal_pio_ok() {
    if !PIO_OK_SIGNALED.swap(true, Ordering::Relaxed) {
        post(KIND_PIO_OK);
    }
}

/// Report one raw byte read out of the i8042 output buffer.
pub fn signal_byte() {
    post(KIND_I8042_BYTE);
}
