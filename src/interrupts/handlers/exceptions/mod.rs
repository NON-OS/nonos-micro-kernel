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

pub mod alignment;
pub mod bound;
pub mod breakpoint;
pub mod context;
pub mod debug;
pub mod device;
pub mod divide;
pub mod double_fault;
pub mod floating_point;
pub mod gpf;
pub mod machine_check;
pub mod nmi;
pub mod opcode;
pub mod overflow;
pub mod page_fault;
pub mod rip_probe;
pub mod segment;
pub mod stack;
pub mod tss;
pub mod virtualization;

pub use alignment::handle as alignment_check;
pub use bound::handle as bound_range_exceeded;
pub use breakpoint::handle as breakpoint;
pub use context::{ExceptionContext, PageFaultContext, PageFaultErrorCode};
pub use debug::{handle as debug, DebugInfo};
pub use device::handle as device_not_available;
pub use divide::handle as divide_error;
pub use double_fault::handle as double_fault;
pub use floating_point::{handle_simd as simd_floating_point, handle_x87 as x87_floating_point};
pub use gpf::handle as general_protection_fault;
pub use machine_check::handle as machine_check;
pub use nmi::{handle as nmi, NmiSource, SYSTEM_CONTROL_PORT_A, SYSTEM_CONTROL_PORT_B};
pub use opcode::handle as invalid_opcode;
pub use overflow::handle as overflow;
pub use page_fault::handle as page_fault;
pub use segment::handle_not_present as segment_not_present;
pub use stack::handle as stack_segment_fault;
pub use tss::handle as invalid_tss;
pub use virtualization::handle as virtualization;
