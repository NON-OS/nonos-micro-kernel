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

//! Poking another CPU.
//!
//! Both calls are best effort and report whether the controller accepted the
//! request. A refusal is real information, not noise: it means the target was
//! not addressable, and a caller that waits for an acknowledgement would
//! otherwise spin until its timeout instead of failing at the point of the
//! mistake.

use super::kind::Ipi;

/// Send `ipi` to the CPU the interrupt controller names `target`.
pub(crate) fn send_ipi(target: u32, ipi: Ipi) -> Result<(), ()> {
    #[cfg(target_arch = "x86_64")]
    return crate::arch::x86_64::interrupt_controller::send_ipi(target, ipi);
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::interrupt_controller::send_ipi(target, ipi);
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        let _ = (target, ipi);
        return Err(());
    }
}

/// Send `ipi` to every CPU except the caller.
pub(crate) fn broadcast_ipi(ipi: Ipi) -> Result<(), ()> {
    #[cfg(target_arch = "x86_64")]
    return crate::arch::x86_64::interrupt_controller::broadcast_ipi(ipi);
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::interrupt_controller::broadcast_ipi(ipi);
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        let _ = ipi;
        return Err(());
    }
}
