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

use super::super::definition::Context;
use crate::arch::x86_64::context::rflags;

impl Context {
    // Keep the saved IF value. These contexts resume at CPL=0: a yield or
    // preempt continuation saved under SFMASK/interrupt-gate discipline
    // carries IF=0 and must stay that way until its own sti/sysret/iretq,
    // or the timer ISR can land on a kernel path that already holds a
    // scheduler spin lock and deadlock the core. User-mode resumes get
    // IF=1 from the iretq frame sanitizer, not from here.
    #[inline]
    pub(in crate::process::context::full) fn sanitize_rflags(rflags: u64) -> u64 {
        rflags::sanitize(rflags)
    }
}
