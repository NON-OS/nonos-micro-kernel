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

use crate::arch::aarch64::boot::info::BootInfo;
use crate::arch::fdt::find::cpus;
use crate::arch::fdt::Fdt;

pub fn populate(fdt: &Fdt, info: &mut BootInfo) {
    let mut cpu_ids = [0u64; 64];
    if let Ok(n) = cpus::find(fdt, &mut cpu_ids) {
        if n > 0 {
            info.cpu_count = n as u32;
        }
    }
}
