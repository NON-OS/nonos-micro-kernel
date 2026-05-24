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

use crate::arch::riscv64::interrupts::cause::{ExceptionCode, TrapCause as RiscvCause};
use crate::arch::riscv64::interrupts::frame::TrapFrame;
use crate::arch::trap::contract::{FaultAccess, PageFaultInfo, TrapCause};

pub(super) fn project(frame: &TrapFrame) -> TrapCause {
    match RiscvCause::from_scause(frame.scause) {
        RiscvCause::Interrupt(_) => TrapCause::Nmi,
        RiscvCause::Exception(code) => exception(code, frame),
    }
}

fn exception(code: ExceptionCode, frame: &TrapFrame) -> TrapCause {
    match code {
        ExceptionCode::LoadPageFault => TrapCause::PageFault(page_fault(frame, FaultAccess::Read)),
        ExceptionCode::StorePageFault => TrapCause::PageFault(page_fault(frame, FaultAccess::Write)),
        ExceptionCode::InstructionPageFault => {
            TrapCause::PageFault(page_fault(frame, FaultAccess::InstructionFetch))
        }
        ExceptionCode::LoadAccessFault
        | ExceptionCode::StoreAccessFault
        | ExceptionCode::InstructionAccessFault => {
            TrapCause::ProtectionFault { error_code: frame.stval as u64 }
        }
        ExceptionCode::IllegalInstruction => TrapCause::InvalidOpcode,
        ExceptionCode::InstructionMisaligned
        | ExceptionCode::LoadMisaligned
        | ExceptionCode::StoreMisaligned => TrapCause::Alignment,
        ExceptionCode::Breakpoint => TrapCause::OtherException(3),
        ExceptionCode::UserEcall => TrapCause::OtherException(8),
        ExceptionCode::SupervisorEcall => TrapCause::OtherException(9),
        ExceptionCode::MachineEcall => TrapCause::OtherException(11),
        ExceptionCode::Unknown(c) => TrapCause::OtherException(c as u8),
    }
}

fn page_fault(frame: &TrapFrame, access: FaultAccess) -> PageFaultInfo {
    PageFaultInfo {
        fault_address: frame.stval as u64,
        access,
        present: false,
        user: frame.is_from_user(),
    }
}
