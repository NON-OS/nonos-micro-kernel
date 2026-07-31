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

/// Hand the exit status to the kernel and never come back.
///
/// AAPCS64 puts the syscall number in x8 and the first argument in x0, which
/// is what the kernel reads out of the trap frame on an `svc`.
pub unsafe fn exit(num: i64, code: i64) -> ! {
    core::arch::asm!(
        "svc #0",
        in("x8") num,
        in("x0") code as u64,
        options(noreturn),
    );
}
