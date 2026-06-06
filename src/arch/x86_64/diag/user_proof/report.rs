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

//! Serial formatting for the pre-iretq audit. One success line and a
//! short failure line. The orchestrator owns the policy; this
//! module owns the bytes.

use super::super::print_hex::print_hex_u64;
use super::gs_state::GsState;

pub(super) fn fail(reason: &[u8], v: u64) {
    crate::sys::serial::print(b"[USER-PROOF] FAIL ");
    crate::sys::serial::print(reason);
    crate::sys::serial::print(b" v=");
    print_hex_u64(v);
    crate::sys::serial::println(b"");
}

pub(super) fn ok(cr3: u64, rip: u64, rsp: u64, rsp0: u64, gs: GsState, df: u64, pf: u64, gp: u64) {
    crate::sys::serial::print(b"[USER-PROOF] OK cr3=");
    print_hex_u64(cr3);
    crate::sys::serial::print(b" rip=");
    print_hex_u64(rip);
    crate::sys::serial::print(b" rsp=");
    print_hex_u64(rsp);
    crate::sys::serial::print(b" rsp0=");
    print_hex_u64(rsp0);
    crate::sys::serial::print(b" gsrsp0=");
    print_hex_u64(gs.rsp0);
    crate::sys::serial::print(b" gs=");
    print_hex_u64(gs.base);
    crate::sys::serial::print(b" kgs=");
    print_hex_u64(gs.kernel_base);
    crate::sys::serial::print(b" istDF=");
    print_hex_u64(df);
    crate::sys::serial::print(b" istPF=");
    print_hex_u64(pf);
    crate::sys::serial::print(b" istGP=");
    print_hex_u64(gp);
    crate::sys::serial::println(b"");
}
