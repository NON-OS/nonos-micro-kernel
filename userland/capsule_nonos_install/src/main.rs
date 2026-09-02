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

//! Install capsule. The program itself is assembly from `_start` to the
//! exit syscall; Rust contributes one translation unit that assembles
//! the sources in dependency order, so every constant and string length
//! is visible to its users, plus the panic contract no_std demands.

#![no_std]
#![no_main]

core::arch::global_asm!(
    include_str!("asm/tags.S"),
    include_str!("asm/strings.S"),
    include_str!("asm/util.S"),
    include_str!("asm/console.S"),
    include_str!("asm/discover.S"),
    include_str!("asm/attest.S"),
    include_str!("asm/steps.S"),
    include_str!("asm/entry.S"),
);

// Unreachable: no Rust code runs. The symbol exists only to satisfy the
// no_std link contract.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
