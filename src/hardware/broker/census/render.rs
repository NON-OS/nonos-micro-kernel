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

use super::line::emit;
use super::lpss::is_lpss_i2c;
use super::verdict::verdict;
use crate::hardware::broker::class::ids;
use crate::hardware::broker::table;
use crate::sys::{boot_log, timer};

const ENABLED: Option<&str> = option_env!("NONOS_DEVICE_CENSUS");
const HOLD_MS: u64 = 25_000;
const SPIN_GUARD: u64 = 30_000_000_000;

// Render the enumerated device table to the framebuffer, then hold the frame
// long enough to photograph before userspace claims the display. The hold reads
// the interrupt-driven clock rather than the TSC busy-delay so it stays honest
// on hardware whose TSC calibration is uncertain. No-op unless the kernel was
// built with NONOS_DEVICE_CENSUS=1.
pub fn render_and_hold() {
    if !matches!(ENABLED, Some("1")) {
        return;
    }
    let devices = table::list();
    boot_log::stage("CENSUS", "device evidence: enumerated hardware");
    let (mut xhci, mut lpss, mut i2c_hid) = (0u32, 0u32, 0u32);
    for rec in &devices {
        emit(rec);
        if rec.class == ids::USB_HOST_XHCI {
            xhci += 1;
        }
        if rec.class == ids::I2C_HID {
            i2c_hid += 1;
        }
        if is_lpss_i2c(rec) {
            lpss += 1;
        }
    }
    verdict("xHCI USB controllers", xhci);
    verdict("Intel LPSS i2c controllers", lpss);
    verdict("i2c-HID touchpad devices", i2c_hid);
    let start = timer::uptime_ms();
    let mut guard: u64 = 0;
    while timer::uptime_ms().wrapping_sub(start) < HOLD_MS {
        core::hint::spin_loop();
        guard = guard.wrapping_add(1);
        if guard > SPIN_GUARD {
            break;
        }
    }
}
