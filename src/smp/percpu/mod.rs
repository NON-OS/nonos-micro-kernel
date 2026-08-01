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

pub mod layout;
pub mod operations;
pub mod types;

pub use types::{PerCpuData, ASID_NONE};

pub use operations::{
    active_asid, current, current_mut, current_process, current_thread, enter_irq, get, in_irq,
    init_ap, init_bsp, kernel_stack, leave_irq, percpu_random, set_active_asid,
    set_current_process, set_current_thread, set_kernel_stack,
};
