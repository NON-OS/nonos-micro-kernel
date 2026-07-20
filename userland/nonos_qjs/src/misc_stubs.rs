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

//! Remaining C ABI: abort and a decimal strtod. QuickJS parses JS numbers with
//! its own routines; strtod covers the C-level decimal-and-exponent cases.

use core::hint::spin_loop;

#[no_mangle]
pub extern "C" fn abs(x: i32) -> i32 {
    x.wrapping_abs()
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    // A QuickJS abort means a fatal engine invariant failed; halt rather than
    // return into corrupted state.
    loop {
        spin_loop();
    }
}

#[no_mangle]
pub unsafe extern "C" fn strtod(s: *const u8, endptr: *mut *mut u8) -> f64 {
    let mut p = s;
    while matches!(*p, b' ' | b'\t' | b'\n' | b'\r') {
        p = p.add(1);
    }
    let start = p;
    let neg = match *p {
        b'-' => {
            p = p.add(1);
            true
        }
        b'+' => {
            p = p.add(1);
            false
        }
        _ => false,
    };
    let mut val = 0.0f64;
    let mut any = false;
    while (*p).is_ascii_digit() {
        val = val * 10.0 + (*p - b'0') as f64;
        p = p.add(1);
        any = true;
    }
    if *p == b'.' {
        p = p.add(1);
        let mut frac = 0.1;
        while (*p).is_ascii_digit() {
            val += (*p - b'0') as f64 * frac;
            frac *= 0.1;
            p = p.add(1);
            any = true;
        }
    }
    if any && matches!(*p, b'e' | b'E') {
        let esave = p;
        p = p.add(1);
        let eneg = match *p {
            b'-' => {
                p = p.add(1);
                true
            }
            b'+' => {
                p = p.add(1);
                false
            }
            _ => false,
        };
        let mut exp = 0i32;
        let mut edig = false;
        while (*p).is_ascii_digit() {
            exp = exp * 10 + (*p - b'0') as i32;
            p = p.add(1);
            edig = true;
        }
        if edig {
            val *= libm::pow(10.0, if eneg { -exp } else { exp } as f64);
        } else {
            p = esave;
        }
    }
    if !endptr.is_null() {
        *endptr = if any { p as *mut u8 } else { start as *mut u8 };
    }
    if neg {
        -val
    } else {
        val
    }
}
