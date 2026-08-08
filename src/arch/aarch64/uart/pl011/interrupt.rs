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

use super::super::state;
use super::device::Pl011;

/// Drain the receive FIFO and acknowledge the interrupt.
///
/// The bytes go nowhere. Nothing in the kernel consumes serial input on any
/// architecture yet, and this handed each one to `drivers::input`, a module
/// that has never existed in the tree, so the handler could not build.
/// Draining is not optional even with no consumer: a receive FIFO left full
/// keeps the interrupt asserted and the CPU never gets out of the handler.
pub fn handle_uart_interrupt() {
    let uart = Pl011::new(state::uart_base());
    while uart.getc().is_some() {}
    uart.clear_interrupts();
}
