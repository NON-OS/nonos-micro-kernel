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

//! Device interrupts and the software syscall vector, which a capsule is
//! running under just as often as the kernel is.
//!
//! A device raises its line whenever the device decides to, which means about
//! as often while CPL=3 is running as while the kernel is. The timer already
//! had a trampoline for exactly this reason; these did not, so every keyboard
//! and mouse interrupt taken from a capsule ran its handler on the user GS
//! base. The handlers ignore the frame, so the shims discard it.

use x86_64::structures::idt::InterruptStackFrame;

use super::tramp_noerr::exc_tramp_noerr;

fn keyboard_handler(_frame: InterruptStackFrame) {
    crate::interrupts::handlers::keyboard();
}
fn mouse_handler(_frame: InterruptStackFrame) {
    crate::interrupts::handlers::mouse();
}
fn syscall_handler(_frame: InterruptStackFrame) {
    crate::interrupts::handlers::syscall();
}

exc_tramp_noerr!(keyboard_trampoline, kbd_trap, keyboard_handler);
exc_tramp_noerr!(mouse_trampoline, mouse_trap, mouse_handler);
exc_tramp_noerr!(int80_trampoline, int80_trap, syscall_handler);
