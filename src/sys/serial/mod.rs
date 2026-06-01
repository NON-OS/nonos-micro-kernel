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

pub mod core;
pub mod print;

pub use core::init;
pub use print::{print, print_dec, print_dec as print_u64, print_hex, print_str, println};

use ::core::sync::atomic::{AtomicBool, Ordering};
static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn set_debug_enabled(v: bool) {
    DEBUG_ENABLED.store(v, Ordering::SeqCst);
}
pub fn is_debug_enabled() -> bool {
    DEBUG_ENABLED.load(Ordering::Relaxed)
}

pub fn trace(s: &[u8]) {
    if is_debug_enabled() {
        print::print(s);
    }
}
pub fn trace_str(s: &str) {
    if is_debug_enabled() {
        print::print_str(s);
    }
}
pub fn traceln(s: &[u8]) {
    if is_debug_enabled() {
        print::println(s);
    }
}
pub fn trace_hex(v: u64) {
    if is_debug_enabled() {
        print::print_hex(v);
    }
}
pub fn trace_dec(v: u64) {
    if is_debug_enabled() {
        print::print_dec(v);
    }
}
