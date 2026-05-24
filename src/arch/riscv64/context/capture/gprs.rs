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

use crate::arch::riscv64::context::types::SavedUser;
use crate::arch::riscv64::interrupts::frame::TrapFrame;

pub fn copy(saved: &mut SavedUser, frame: &TrapFrame) {
    saved.gprs[0] = frame.ra as u64;
    saved.gprs[1] = frame.sp as u64;
    saved.gprs[2] = frame.gp as u64;
    saved.gprs[3] = frame.tp as u64;
    saved.gprs[4] = frame.t0 as u64;
    saved.gprs[5] = frame.t1 as u64;
    saved.gprs[6] = frame.t2 as u64;
    saved.gprs[7] = frame.s0 as u64;
    saved.gprs[8] = frame.s1 as u64;
    saved.gprs[9] = frame.a0 as u64;
    saved.gprs[10] = frame.a1 as u64;
    saved.gprs[11] = frame.a2 as u64;
    saved.gprs[12] = frame.a3 as u64;
    saved.gprs[13] = frame.a4 as u64;
    saved.gprs[14] = frame.a5 as u64;
    saved.gprs[15] = frame.a6 as u64;
    saved.gprs[16] = frame.a7 as u64;
    saved.gprs[17] = frame.s2 as u64;
    saved.gprs[18] = frame.s3 as u64;
    saved.gprs[19] = frame.s4 as u64;
    saved.gprs[20] = frame.s5 as u64;
    saved.gprs[21] = frame.s6 as u64;
    saved.gprs[22] = frame.s7 as u64;
    saved.gprs[23] = frame.s8 as u64;
    saved.gprs[24] = frame.s9 as u64;
    saved.gprs[25] = frame.s10 as u64;
    saved.gprs[26] = frame.s11 as u64;
    saved.gprs[27] = frame.t3 as u64;
    saved.gprs[28] = frame.t4 as u64;
    saved.gprs[29] = frame.t5 as u64;
    saved.gprs[30] = frame.t6 as u64;
}
