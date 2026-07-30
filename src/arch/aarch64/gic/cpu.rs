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
use super::redistributor;
use super::state::redist_base;
use crate::sys::serial;

/// Bring up the per-CPU half of the GIC on the calling core.
pub fn init_gic_cpu() {
    match redistributor::for_this_cpu(redist_base()) {
        Some(redist) => redist.init(),
        // Without its redistributor this CPU can neither wake for SGIs nor
        // enable its private interrupts, so the timer tick will not arrive.
        // Say so; a silent miss looks like a hung scheduler later.
        None => serial::println(b"[GIC] no redistributor claims this CPU"),
    }

    icc::init();
}
