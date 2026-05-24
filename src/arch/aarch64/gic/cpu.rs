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

use super::icc;
use super::redistributor::GicRedistributor;
use super::state::redist_base;

pub fn init_gic_cpu() {
    let redist_base = redist_base();

    if redist_base != 0 {
        let cpu_id = crate::arch::aarch64::cpu::cpu_id();
        let redist_offset = cpu_id as u64 * 0x20000;
        let redist = GicRedistributor::new(redist_base + redist_offset);
        redist.init();
    }

    icc::init();
}
