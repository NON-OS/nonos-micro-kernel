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

use super::log::log_record;
use super::status::{clear_status, has_faults, overflowed};
use super::take::take_fault;
use crate::arch::x86_64::iommu::regs::cap;
use crate::arch::x86_64::iommu::unit::report::probed;
use crate::sys::serial;

/// Drain every pending fault record to the console and return how many.
///
/// A denial the operator cannot see is indistinguishable from a hang, so this
/// is what makes enforcement diagnosable: a device that stops working under
/// translation names itself here.
pub fn drain_faults() -> usize {
    let Some(info) = probed() else {
        return 0;
    };
    if !has_faults(&info.unit) {
        return 0;
    }
    if overflowed(&info.unit) {
        serial::println(b"[VT-D] fault records overflowed; some denials were lost");
    }

    let mut drained = 0;
    for index in 0..cap::fault_recording_count(info.cap) as usize {
        if let Some(record) = take_fault(&info.unit, info.cap, index) {
            log_record(&record);
            drained += 1;
        }
    }
    // SAFETY: eK@nonos.systems - every record was taken above, so no pending
    // record is left with nothing advertising it.
    unsafe {
        clear_status(&info.unit);
    }
    drained
}
