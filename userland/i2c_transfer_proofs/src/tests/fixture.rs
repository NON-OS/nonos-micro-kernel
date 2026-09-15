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

//! A driver bound to a modelled controller with a touchpad on its bus.

use nonos_i2cmodel::{touchpad, Bus, Config, Designware, HidOverI2c, Shared};

use crate::bench::{self, GEMINI_LAKE_HZ};
use crate::driver::Driver;
use crate::regs::{self, Attached};

/// The touchpad's address on the HP this driver was brought up against.
pub const PAD: u8 = 0x15;
pub const CLOCK_HZ: u32 = GEMINI_LAKE_HZ;
/// ELAN vendor id and a product the driver has bound to.
pub const VENDOR: u16 = 0x04F3;
pub const PRODUCT: u16 = 0x3028;
/// wMaxInputLength: two bytes of length plus the eight-byte touch report.
pub const MAX_INPUT: u16 = 2 + touchpad::TOUCH_REPORT_LEN;

pub struct Bench {
    pub driver: Driver,
    pub pad: Shared<HidOverI2c>,
    _attached: Attached,
}

/// The touchpad as it answers when healthy.
pub fn pad() -> HidOverI2c {
    HidOverI2c::new(PAD, touchpad::report_descriptor(), MAX_INPUT, VENDOR, PRODUCT)
}

pub fn bench(config: Config) -> Bench {
    bench_with(config, pad())
}

/// The driver after its own bring-up ran against a core of `config` with
/// `device` on the bus, bound to it the way setup binds after a probe.
pub fn bench_with(config: Config, device: HidOverI2c) -> Bench {
    let pad = Shared::new(device);
    let attached = regs::attach(Designware::new(config, Bus::default().with(pad.clone())));
    let driver = bench::driver(CLOCK_HZ, PAD).expect("bring-up against the modelled core");
    Bench { driver, pad, _attached: attached }
}

/// The core, for a look at the wire or the violation list.
pub fn core<R>(f: impl FnOnce(&mut Designware) -> R) -> R {
    regs::with(f)
}
