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

use super::hooks;
use super::state;

pub fn on_timer_interrupt() {
    state::increment_ticks();
    if option_env!("NONOS_FBCONSOLE").is_some() {
        super::heartbeat::on_tick(state::get_ticks());
    }
    crate::sched::tick();
    #[cfg(feature = "input-probe-inject")]
    crate::kernel_core::surface_registry::inject::on_tick();
    // The legacy in-kernel network stack runs its retransmit/timeout
    // wheel from the timer tick. The microkernel has no in-kernel
    // sockets; capsule-side networking, when present, drives its own
    // timers via IPC.
    crate::sched::scheduler::process::check_sleeping_processes();

    if state::get_ticks() % 10 == 0 {
        crate::process::alarm::tick();
    }

    #[cfg(all(target_arch = "x86_64", feature = "nonos-arch-iommu"))]
    crate::arch::x86_64::iommu::unit::fault::poll_faults(state::get_ticks());

    hooks::invoke_hook();

    if crate::sched::scheduler::preemption::need_reschedule() {
        crate::sched::scheduler::preemption::clear_reschedule();
        if crate::process::scheduler::contract::switch(
            crate::process::scheduler::contract::SwitchIntent::Preempt,
        )
        .is_err()
        {
            return;
        }
    }
}

pub fn tick() {
    on_timer_interrupt();
}
