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

static POST_PHASE: AtomicU32 = AtomicU32::new(0);
const POST_X0: u32 = 54;

// Advanced once per input event a driver hands the kernel via
// mk_input_event_post. Frozen while you press keys or drag the touchpad means no
// driver is decoding and posting events: the break is in the driver or its
// hardware access, upstream of delivery. Changing means a driver is producing
// events and any remaining problem is downstream.
pub fn input_post_blip() {
    paint_blip(POST_X0, POST_PHASE.fetch_add(1, Ordering::Relaxed));
}
