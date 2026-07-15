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

#[inline]
pub fn halt_loop() -> ! {
    crate::arch::halt_loop()
}

#[inline]
pub fn halt() {
    // Single-shot HLT, used by the idle hook before the panic path.
    // The cross-arch `cpu_yield` is the same primitive.
    crate::arch::cpu_yield();
}

#[inline]
pub fn disable_interrupts() {
    crate::arch::disable_interrupts()
}

#[inline]
pub fn enable_interrupts() {
    crate::arch::enable_interrupts()
}

#[inline]
pub fn interrupts_enabled() -> bool {
    crate::arch::cpu::interrupts_enabled()
}

pub fn without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let enabled = interrupts_enabled();
    if enabled {
        disable_interrupts();
    }

    let result = f();

    if enabled {
        enable_interrupts();
    }

    result
}
