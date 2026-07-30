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

//! x86_64 backend for `arch::interrupt_controller`: the local APIC.

mod ops;
mod vector;

pub use ops::{broadcast_ipi, end_of_interrupt, local_id, send_ipi};
pub use vector::{
    vector_of, IPI_BARRIER, IPI_CALL_FUNCTION, IPI_PANIC, IPI_RESCHEDULE, IPI_STOP,
    IPI_TLB_SHOOTDOWN,
};
