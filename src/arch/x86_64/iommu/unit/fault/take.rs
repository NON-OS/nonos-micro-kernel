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

use super::record::FaultRecord;
use crate::arch::x86_64::iommu::regs::{cap, offsets};
use crate::arch::x86_64::iommu::unit::access::RemapUnit;

/// Read one record and free its slot. The hardware stops recording once its
/// records are full, so a full ring would hide every later attempt.
///
/// The record is read before its fault bit is cleared, so a racing caller can
/// see one twice but never lose one.
pub(super) fn take_fault(
    unit: &RemapUnit,
    cap_word: u64,
    index: usize,
) -> Option<FaultRecord> {
    let base = cap::fault_recording_offset(cap_word) + index * 16;
    let high = unit.read64(base + 8);
    if high & offsets::FRCD_FAULT == 0 {
        return None;
    }
    let record = FaultRecord {
        source: offsets::frcd_source(high),
        address: unit.read64(base),
        reason: offsets::frcd_reason(high),
        read: high & offsets::FRCD_TYPE_READ != 0,
    };
    // SAFETY: eK@nonos.systems - write-one-to-clear frees reporting space and
    // changes nothing about what any device may reach.
    unsafe {
        unit.write64(base + 8, offsets::FRCD_FAULT);
    }
    Some(record)
}
