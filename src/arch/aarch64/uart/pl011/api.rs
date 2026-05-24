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

fn current() -> Pl011 {
    Pl011::new(state::uart_base())
}

pub fn init_uart(base: u64) {
    state::set_uart_base(base);
    if Pl011::new(base).init(115200, 24_000_000).is_err() {
        crate::arch::aarch64::cpu::halt();
    }
}

pub fn putc(c: char) {
    current().putc(c as u8);
}

pub fn puts(s: &[u8]) {
    current().puts(s);
}

pub fn getc() -> Option<u8> {
    current().getc()
}
