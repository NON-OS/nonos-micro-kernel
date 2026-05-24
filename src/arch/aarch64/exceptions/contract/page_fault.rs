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

use crate::arch::aarch64::exceptions::frame::ExceptionFrame;
use crate::arch::trap::contract::{FaultAccess, PageFaultInfo};



pub(super) fn decode_data(frame: &ExceptionFrame) -> PageFaultInfo {
    let iss = (frame.esr & 0x01FF_FFFF) as u32;
    let wnr = (iss & (1 << 6)) != 0;
    let dfsc = (iss & 0x3F) as u8;
    PageFaultInfo {
        fault_address: frame.far,
        access: if wnr { FaultAccess::Write } else { FaultAccess::Read },
        present: !is_translation_fault(dfsc),
        user: frame.is_from_el0(),
    }
}

pub(super) fn decode_instruction(frame: &ExceptionFrame) -> PageFaultInfo {
    let iss = (frame.esr & 0x01FF_FFFF) as u32;
    let ifsc = (iss & 0x3F) as u8;
    PageFaultInfo {
        fault_address: frame.far,
        access: FaultAccess::InstructionFetch,
        present: !is_translation_fault(ifsc),
        user: frame.is_from_el0(),
    }
}




#[inline]
fn is_translation_fault(fsc: u8) -> bool {
    (fsc & 0x3C) == 0x04
}
