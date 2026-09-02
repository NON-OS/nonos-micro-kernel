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

//! The trampoline symbols defined in `src/arch/x86_64/asm/exceptions.S`.
//!
//! Each is the raw vector entry the IDT installs: it swaps gs when the
//! interrupted CPL was user, saves the full register state, hands the frame
//! (and error code where the vector pushes one) to its shim, and unwinds by
//! the exact inverse. The IDT registration takes these as addresses, so the
//! declarations carry no Rust-visible signature beyond existence.

extern "C" {
    pub fn de_trampoline();
    pub fn db_trampoline();
    pub fn bp_trampoline();
    pub fn of_trampoline();
    pub fn br_trampoline();
    pub fn ud_trampoline();
    pub fn nm_trampoline();
    pub fn mf_trampoline();
    pub fn xf_trampoline();
    pub fn ve_trampoline();
    pub fn ts_trampoline();
    pub fn np_trampoline();
    pub fn ss_trampoline();
    pub fn gpf_trampoline();
    pub fn ac_trampoline();
    pub fn keyboard_trampoline();
    pub fn mouse_trampoline();
    pub fn int80_trampoline();
}
