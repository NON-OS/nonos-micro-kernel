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

use super::deadline::{default_tick_delta, set_next_timer};

pub fn handle_timer_interrupt() {
    if set_next_timer(default_tick_delta()).is_err() {
        super::super::cpu::halt();
    }
    if super::super::cpu::csr::clear_csr(
        super::super::cpu::csr::SIP,
        super::super::cpu::csr::SIP_STIP,
    )
    .is_err()
    {
        super::super::cpu::halt();
    }
    crate::process::scheduler::preemption::tick::tick();
}
