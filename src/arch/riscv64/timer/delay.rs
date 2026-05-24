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

use super::convert::current_time_ns;

pub fn delay_ns(ns: u64) {
    let start = current_time_ns();
    while current_time_ns().saturating_sub(start) < ns {
        core::hint::spin_loop();
    }
}

pub fn delay_us(us: u64) {
    delay_ns(us.saturating_mul(1000));
}

pub fn delay_ms(ms: u64) {
    delay_ns(ms.saturating_mul(1_000_000));
}
