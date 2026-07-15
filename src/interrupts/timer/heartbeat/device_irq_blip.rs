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

use core::sync::atomic::{AtomicU32, Ordering};

use super::paint_blip::paint_blip;

// A second, independent marker to the right of the timer heartbeat, advanced
// once per delivered device interrupt (keyboard, mouse, touchpad, any broker
// IRQ). Frozen = no device interrupt has ever arrived; changing when you
// press a key = the IRQ is being delivered and the problem is downstream in
// the driver. Painted from the broker dispatch, which is lock free and gs
// free, so this stays raw framebuffer writes with no allocation.
static IRQ_PHASE: AtomicU32 = AtomicU32::new(0);
const IRQ_X0: u32 = 30;

pub fn device_irq_blip() {
    paint_blip(IRQ_X0, IRQ_PHASE.fetch_add(1, Ordering::Relaxed));
}
