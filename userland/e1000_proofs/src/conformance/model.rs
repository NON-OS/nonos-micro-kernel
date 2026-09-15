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

//! A BAR0 window and the part that answers from it.

use std::sync::{Arc, Mutex, MutexGuard};

use nonos_devmodel::{run, FakeBar, LiveDevice};

use crate::constants::regs::{REG_CTRL, REG_RAH0, REG_RAL0};
use crate::constants::status::{CTRL_RST, RAH_AV};
use crate::constants::MAC_LEN;

/// The address burned into the part, loaded into RAL0/RAH0 at power-on.
pub const EEPROM: [u8; MAC_LEN] = [0x00, 0x1B, 0x21, 0x11, 0x22, 0x33];

pub fn window() -> Arc<FakeBar> {
    let bar = Arc::new(FakeBar::new(0x6000));
    let low = u32::from_le_bytes([EEPROM[0], EEPROM[1], EEPROM[2], EEPROM[3]]);
    let high = u32::from_le_bytes([EEPROM[4], EEPROM[5], 0, 0]) | RAH_AV;
    bar.present32(REG_RAL0, low);
    bar.present32(REG_RAH0, high);
    bar
}

/// The receive address the part would filter on, from RAL0/RAH0.
pub fn receive_address(bar: &FakeBar) -> [u8; MAC_LEN] {
    let low = bar.wrote32(REG_RAL0).to_le_bytes();
    let high = bar.wrote32(REG_RAH0).to_le_bytes();
    [low[0], low[1], low[2], low[3], high[0], high[1]]
}

/// A part that completes a reset on the next look.
pub fn resetting_part(bar: &FakeBar) {
    let ctrl = bar.wrote32(REG_CTRL);
    if ctrl & CTRL_RST != 0 {
        bar.present32(REG_CTRL, ctrl & !CTRL_RST);
    }
}

static TURN: Mutex<()> = Mutex::new(());

/// A running part and the turn it holds. The driver waits a fixed number of
/// spins for the part to answer, and on a loaded runner a model thread that
/// shares the machine with a dozen other tests can be preempted for longer
/// than that budget. One live test at a time keeps the part scheduled.
pub struct Live {
    _part: LiveDevice,
    _turn: MutexGuard<'static, ()>,
}

/// The resetting part, running, proven to answer, and alone on the machine.
pub fn live_part(bar: &Arc<FakeBar>) -> Live {
    let turn = TURN.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let part = run(bar, resetting_part);
    bar.present32(REG_CTRL, CTRL_RST);
    while bar.wrote32(REG_CTRL) & CTRL_RST != 0 {
        std::thread::yield_now();
    }
    Live { _part: part, _turn: turn }
}
