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

//! The windows the driver is pointed at.
//!
//! One window carries the modern common configuration at offset zero, the
//! notify region at `NOTIFY_AT` and the device configuration at `DEVICE_AT`,
//! the way three BAR-relative capabilities would. Modern feature pages read
//! from one register after a select write, and a passive window cannot tell
//! the pages apart, so whatever is presented there is what both pages offer:
//! bit 0 is VIRGL on the low page and VERSION_1 on the high one.

use std::sync::Arc;

use nonos_devmodel::FakeBar;

use crate::constants::{
    LEG_HOST_FEATURES, LEG_QUEUE_NUM, MOD_DEVICE_FEATURE, MOD_QUEUE_NOTIFY_OFF, MOD_QUEUE_SIZE,
};
use crate::regs::Regs;

pub const NOTIFY_AT: usize = 0x100;
pub const NOTIFY_MULT: usize = 4;
pub const NOTIFY_OFF: u16 = 3;
pub const DEVICE_AT: usize = 0x200;
pub const QUEUE_SIZE: u16 = 64;
/// Where a bring-up test says the ring region lives; nothing reads it.
pub const REGION_PHYS: u64 = 0x1_2345_0000;

pub fn modern_window(offered: u32, queue_size: u16) -> Arc<FakeBar> {
    let bar = Arc::new(FakeBar::new(0x300));
    bar.present32(MOD_DEVICE_FEATURE, offered);
    bar.present16(MOD_QUEUE_SIZE, queue_size);
    bar.present16(MOD_QUEUE_NOTIFY_OFF, NOTIFY_OFF);
    bar
}

pub fn modern_regs(bar: &FakeBar) -> Regs {
    let b = bar.base();
    Regs::modern(b, 0, b, NOTIFY_AT, NOTIFY_MULT, b, DEVICE_AT)
}

pub fn legacy_window(offered: u32, queue_num: u16) -> Arc<FakeBar> {
    let bar = Arc::new(FakeBar::new(0x40));
    bar.present32(LEG_HOST_FEATURES, offered);
    bar.present16(LEG_QUEUE_NUM, queue_num);
    bar
}
