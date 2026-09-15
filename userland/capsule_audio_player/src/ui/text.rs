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

//! String shaping for the screens: measured truncation and the small set of
//! formatters the comps ask for. Truncation cuts on char boundaries and
//! measures with the same function the painter draws with.

extern crate alloc;

use alloc::string::String;
use nonos_app_skeleton::measure_ttf;

pub fn truncate_to_width(s: &str, px: f32, max: i32) -> String {
    if measure_ttf(s, px) <= max {
        return String::from(s);
    }
    let ell = "...";
    let budget = max - measure_ttf(ell, px);
    if budget <= 0 {
        return String::new();
    }
    let mut cut = 0;
    for (i, _) in s.char_indices() {
        if measure_ttf(&s[..i], px) > budget {
            break;
        }
        cut = i;
    }
    let mut out = String::from(&s[..cut]);
    out.push_str(ell);
    out
}

pub fn push_u32(out: &mut String, mut v: u32, mut pad: usize) {
    let mut d = [0u8; 10];
    let mut n = 0;
    loop {
        d[n] = b'0' + (v % 10) as u8;
        v /= 10;
        n += 1;
        if v == 0 {
            break;
        }
    }
    while pad > n {
        out.push('0');
        pad -= 1;
    }
    while n > 0 {
        n -= 1;
        out.push(d[n] as char);
    }
}

pub fn mmss(ms: u32) -> String {
    let total = ms / 1000;
    let mut s = String::new();
    push_u32(&mut s, total / 60, 1);
    s.push(':');
    push_u32(&mut s, total % 60, 2);
    s
}

pub fn count(n: usize, one: &str, many: &str) -> String {
    let mut s = String::new();
    push_u32(&mut s, n as u32, 1);
    s.push(' ');
    s.push_str(if n == 1 { one } else { many });
    s
}

pub fn upper(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        out.push(c.to_ascii_uppercase());
    }
    out
}
