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

use super::state::SCHEDULER_STATS;
use crate::arch::x86_64::idt::without_interrupts;
use crate::process::scheduler::contract::{switch as contract_switch, SwitchIntent};
use core::sync::atomic::Ordering;

pub fn yield_now() {
    SCHEDULER_STATS.voluntary_yields.fetch_add(1, Ordering::Relaxed);
    without_interrupts(|| {
        let _ = contract_switch(SwitchIntent::Yield);
    });
}
