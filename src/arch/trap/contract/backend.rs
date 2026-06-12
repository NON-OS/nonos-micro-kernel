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

use super::cause::TrapCause;
use super::frame::TrapFrame;

#[cfg(target_arch = "aarch64")]
use super::backend_aarch64 as imp;
#[cfg(target_arch = "riscv64")]
use super::backend_riscv64 as imp;
#[cfg(target_arch = "x86_64")]
use super::backend_x86_64 as imp;

pub(super) fn report_fatal<F: TrapFrame>(frame: &F, cause: &TrapCause) {
    imp::report_fatal(frame, cause)
}

pub(super) fn halt_forever() -> ! {
    imp::halt_forever()
}
