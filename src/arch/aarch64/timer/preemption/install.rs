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

use crate::arch::aarch64::gic::{enable_irq, register_irq_handler};
use crate::arch::aarch64::timer::set_timer;

use super::handler::{timer_tick, TICK_PERIOD_NS};
use super::state::phys_intid;

pub fn install_on_cpu() -> Result<(), &'static str> {
    let intid = phys_intid();
    if intid == 0 {
        return Err("timer intid unset (DTB /timer node missing or unparsed)");
    }
    register_irq_handler(intid, timer_tick).map_err(|_| "gic timer irq registration failed")?;
    enable_irq(intid);
    set_timer(TICK_PERIOD_NS);
    Ok(())
}
