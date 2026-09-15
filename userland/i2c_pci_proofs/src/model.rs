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

//! The windows the tests put in front of the driver.
//!
//! A passive window reads back what was written, which is what the
//! DesignWare core does for its configuration registers. What it does not do
//! is move IC_ENABLE_STATUS when IC_ENABLE is written, so a test that wants
//! the driver to see the controller as stuck enabled, or as already
//! quiescent, presents that status itself and the driver's wait loops behave
//! exactly as they would against a part in that state.

use nonos_devmodel::FakeBar;

use crate::constants::{IC_COMP_TYPE, IC_ENABLE, IC_ENABLE_STATUS};
use crate::init::InitState;

/// The message a bring-up gave up with. `InitState` carries no `Debug`, so
/// `expect_err` is not available and the success case is spelled out here.
pub fn refusal(outcome: Result<InitState, &'static str>) -> &'static str {
    match outcome {
        Ok(_) => panic!("bring-up succeeded where this window should have refused it"),
        Err(message) => message,
    }
}

/// Wide enough for the LPSS private block at 0x200, past the core's 0x100.
pub const WINDOW: usize = 0x400;
/// Gemini Lake's I2C input clock, the part this capsule was brought up on.
pub const CLOCK_HZ: u32 = 133_000_000;
/// The DesignWare I2C component signature a live core answers with.
pub const DW_COMP_TYPE: u32 = 0x4457_0140;

/// A window every byte of which reads back `byte`, as an undecoded or
/// unpowered BAR does.
pub fn stuck_at(byte: u8) -> FakeBar {
    let bar = FakeBar::new(WINDOW);
    for offset in 0..WINDOW {
        bar.present8(offset, byte);
    }
    bar
}

/// A powered core that answers with its signature and reports itself already
/// disabled, so the driver's disable wait clears on the first read.
pub fn live() -> FakeBar {
    let bar = FakeBar::new(WINDOW);
    bar.present32(IC_COMP_TYPE as usize, DW_COMP_TYPE);
    bar
}

/// A powered core that never lets go of its enable bit. IC_ENABLE_STATUS
/// stays set whatever the driver writes to IC_ENABLE, which is how a part
/// mid-transfer, or one whose functional clock is gated, behaves.
pub fn stuck_enabled() -> FakeBar {
    let bar = live();
    bar.present32(IC_ENABLE as usize, 1);
    bar.present32(IC_ENABLE_STATUS as usize, 1);
    bar
}
