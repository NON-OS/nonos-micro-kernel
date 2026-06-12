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

mod exception_level;
mod registers;
mod snapshot;

pub use exception_level::{current_el, is_el1, is_el2, is_el3, ExceptionLevel};
pub use registers::{
    read_daif, read_elr_el1, read_esr_el1, read_far_el1, read_lr, read_nzcv, read_pc, read_sp,
    read_spsr_el1,
};
pub use snapshot::CpuState;
