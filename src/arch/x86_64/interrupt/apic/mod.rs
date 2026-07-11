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

mod adopt;
pub mod constants;
pub mod error;
pub mod init;
mod init_ap;
mod init_x2apic;
mod init_xapic;
pub mod ipi;
mod ipi_ap;
mod ipi_basic;
pub mod mmio;
pub mod ops;
mod ops_core;
mod ops_status;
pub mod preemption;
pub mod state;
pub mod timer;
mod timer_mask;
mod timer_mode;
mod timer_ops;

pub use adopt::adopt_bsp_state;
pub use constants::{VEC_ERROR, VEC_SPURIOUS, VEC_THERMAL, VEC_TIMER};
pub use error::{ApicError, ApicResult};
pub use init::{init, init_apic};
pub use init_ap::init_ap_lapic;
pub use ipi::{ipi_all, ipi_one, ipi_others, ipi_self, start_ap};
pub use ops::{eoi, get_tpr, id, max_lvt, send_eoi, set_tpr, status, version, ApicStatus};
pub use state::{
    has_tsc_deadline, has_x2apic, has_xapic, is_initialized, is_x2apic, supports_tsc_deadline,
};
pub use timer::{
    calibrate_timer, divider_to_code, timer_current, timer_deadline_tsc, timer_enable, timer_mask,
    timer_oneshot, timer_unmask, TimerMode,
};
