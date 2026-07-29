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

//! Bringing the secondary CPUs online.
//!
//! The two architectures do this at different times, not just in different
//! ways. A PC has to build a real-mode trampoline, identity-map it, and walk
//! each core through INIT-SIPI-SIPI from kernel-core, because nothing before
//! kernel-core can allocate. An ARM board asks firmware to do it through PSCI,
//! which needs no trampoline and no allocator, so the aarch64 boot path has
//! already released its cores by the time this runs.
//!
//! Returning the count of cores already running is therefore the honest answer
//! on aarch64, not a stub: the work is done, and reporting zero would tell the
//! scheduler it is on a uniprocessor.

/// Number of secondaries brought online, not counting the boot CPU.
pub fn start_aps() -> Result<usize, &'static str> {
    #[cfg(target_arch = "x86_64")]
    return super::ap_start::start_aps();

    #[cfg(target_arch = "aarch64")]
    {
        let online = crate::arch::aarch64::boot::multicore::online_cpu_count() as usize;
        crate::smp::state::CPUS_ONLINE.store(online, core::sync::atomic::Ordering::Release);
        Ok(online.saturating_sub(1))
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    Ok(0)
}
