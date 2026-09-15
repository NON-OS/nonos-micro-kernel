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

//! A core with a touchpad on its bus, in the states the tests start from.

use crate::designware::regs::{IC_CON, IC_ENABLE, IC_TAR, LPSS_PRIV_RESETS, LPSS_RESETS_RELEASED};
use crate::{touchpad, Bus, Config, Designware, HidOverI2c, Shared};

pub(super) const PAD: u8 = 0x15;

pub(super) fn parts(config: Config) -> (Designware, Shared<HidOverI2c>) {
    let pad = Shared::new(HidOverI2c::new(PAD, touchpad::report_descriptor(), 10, 0x04F3, 0x3028));
    (Designware::new(config, Bus::default().with(pad.clone())), pad)
}

/// A core released from reset, configured with `con`, aimed at the pad and
/// enabled: what the driver's bring-up plus a target change leave behind.
pub(super) fn ready(config: Config, con: u32) -> (Designware, Shared<HidOverI2c>) {
    let (mut dw, pad) = parts(config);
    dw.write32(LPSS_PRIV_RESETS, LPSS_RESETS_RELEASED);
    dw.write32(IC_CON, con);
    dw.write32(IC_TAR, PAD as u32);
    dw.write32(IC_ENABLE, 1);
    (dw, pad)
}
