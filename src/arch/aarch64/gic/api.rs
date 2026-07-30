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

use super::device::Gic;
use super::state::{dist_base, redist_base, set_bases};

pub fn init_gic(dist: u64, redist: u64) {
    set_bases(dist, redist);
    let mut gic = Gic::new(dist, redist);
    gic.init();
}

pub fn enable_irq(irq: u32) {
    Gic::new(dist_base(), redist_base()).enable_irq(irq);
}

pub fn disable_irq(irq: u32) {
    Gic::new(dist_base(), redist_base()).disable_irq(irq);
}

pub fn send_sgi(target: u32, intid: u32) -> Result<(), ()> {
    super::icc::send_sgi(target, intid)
}

pub fn send_sgi_all_others(intid: u32) -> Result<(), ()> {
    super::icc::send_sgi_all_others(intid)
}
