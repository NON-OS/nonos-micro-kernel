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

use crate::interrupts::apic;
use crate::interrupts::pic;

const TIMER_IRQ_LINE: u8 = 0;

pub(super) fn send_eoi() {
    if apic::is_enabled() {
        apic::send_eoi();
    } else {
        pic::send_eoi(TIMER_IRQ_LINE);
    }
}
