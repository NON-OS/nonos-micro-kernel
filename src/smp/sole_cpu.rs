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

use super::state::{BSP_APIC_ID, CPU_COUNT, SMP_INITIALIZED};
use core::sync::atomic::Ordering;

/// The APIC id of the only CPU that can be executing, or `None` when more
/// than one is possible and the caller must ask the hardware.
///
/// `init_bsp` records the BSP's APIC id and the detected CPU count before it
/// publishes `SMP_INITIALIZED`, and `ap_start` brings no AP up when that
/// count is one. So once the flag reads back set with a count of one, the
/// BSP is the only CPU that can reach this and its id is already known. The
/// acquire load orders the two release stores that precede the flag.
#[inline]
pub fn sole_cpu_apic_id() -> Option<u32> {
    if !SMP_INITIALIZED.load(Ordering::Acquire) {
        return None;
    }
    if CPU_COUNT.load(Ordering::Relaxed) > 1 {
        return None;
    }
    Some(BSP_APIC_ID.load(Ordering::Relaxed))
}
