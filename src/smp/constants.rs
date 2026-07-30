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

pub const MAX_CPUS: usize = 256;

pub const PERCPU_STACK_SIZE: usize = 64 * 1024;

pub const AP_TRAMPOLINE_ADDR: u64 = 0x8000;

pub const IPI_FLAG_TLB_SHOOTDOWN: u32 = 1 << 0;
pub const IPI_FLAG_RESCHEDULE: u32 = 1 << 1;
pub const IPI_FLAG_PANIC: u32 = 1 << 2;
pub const IPI_FLAG_STOP: u32 = 1 << 3;
