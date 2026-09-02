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

//! The exception and interrupt trampolines, as assembly.
//!
//! Every entry from a running context into the kernel used to be a
//! macro-generated naked function; the instructions now live written out in
//! `src/arch/x86_64/asm/exceptions.S`, one explicit entry per vector, so the
//! code that decides swapgs and saves the interrupted state is a single file
//! read top to bottom with no generation in between. `entries` declares the
//! trampoline symbols the IDT installs; `shims` holds the C functions each
//! trampoline calls, which do nothing but read the frame and forward to the
//! handler.

mod entries;
mod shims;

pub use entries::{
    ac_trampoline, bp_trampoline, br_trampoline, db_trampoline, de_trampoline, gpf_trampoline,
    int80_trampoline, keyboard_trampoline, mf_trampoline, mouse_trampoline, nm_trampoline,
    np_trampoline, of_trampoline, ss_trampoline, ts_trampoline, ud_trampoline, ve_trampoline,
    xf_trampoline,
};
