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
use nonos_libc::mk_device_release;

use super::bring_up_one::bring_up_one;
use crate::constants::controller_index;
use crate::discover::{
    find_controllers, find_touchpad_addrs, AcpiTouchpad, Found, MAX_CONTROLLERS, MAX_TARGETS,
};
use crate::driver::Driver;
use crate::transaction::probe_hid;

pub fn run() -> Result<Driver, &'static str> {
    let mut cands = [Found::default(); MAX_CONTROLLERS];
    let n = find_controllers(&mut cands);
    if n == 0 {
        return Err("i2c-pci: controller not found");
    }
    // Several LPSS I2C hosts are present on Gemini Lake and only one carries
    // the touchpad — and multi-SKU firmware declares several candidate pads of
    // which only the fitted one exists. Bind the controller whose bus actually
    // answers one of the candidate addresses, and remember which address it
    // was so the HID driver talks to the device that is really there.
    let mut targets = [AcpiTouchpad::default(); MAX_TARGETS];
    let t = find_touchpad_addrs(&mut targets);
    for dev in &cands[..n] {
        if let Ok(mut driver) = bring_up_one(*dev) {
            if t == 0 {
                return Ok(driver);
            }
            let hit = targets[..t].iter().find(|tg| probe_hid(&driver, tg.addr).unwrap_or(false));
            if let Some(tg) = hit {
                driver.bound_by_probe = true;
                driver.bound_addr = tg.addr;
                return Ok(driver);
            }
            let _ = mk_device_release(driver.device_id);
        }
    }
    // No bus answered any candidate, which this early in boot can just mean
    // the device is still asleep. Prefer a controller the firmware named in a
    // candidate's _CRS over a blind first-fit; the HID driver keeps reprobing
    // until the device wakes.
    for tg in &targets[..t] {
        let Some(idx) = tg.controller_idx else { continue };
        for dev in &cands[..n] {
            if !dev.is_acpi && controller_index(dev.pci_device) == Some(idx) {
                if let Ok(mut driver) = bring_up_one(*dev) {
                    driver.bound_addr = tg.addr;
                    return Ok(driver);
                }
            }
        }
    }
    // Last resort so a single-controller box still works.
    for dev in &cands[..n] {
        if let Ok(driver) = bring_up_one(*dev) {
            return Ok(driver);
        }
    }
    Err("i2c-pci: no controller answered")
}
