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

static DRAIN_PHASE: AtomicU32 = AtomicU32::new(0);
const DRAIN_X0: u32 = 78;

// Advanced once per batch the input router drains from the kernel ring via
// mk_input_event_drain. If the post marker changes but this one stays frozen,
// the router is not consuming events and they never reach the compositor/cursor.
pub fn input_drain_blip() {
    paint_blip(DRAIN_X0, DRAIN_PHASE.fetch_add(1, Ordering::Relaxed));
}
