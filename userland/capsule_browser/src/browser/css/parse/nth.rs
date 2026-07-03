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

use alloc::string::String;

// The An+B argument of :nth-child(): "odd", "even", "3", "2n", "2n+1",
// "-n+2". Returns (a, b).
pub(super) fn parse_nth(arg: &str) -> Option<(i32, i32)> {
    let s: String = arg.to_ascii_lowercase().split_whitespace().collect();
    match s.as_str() {
        "odd" => return Some((2, 1)),
        "even" => return Some((2, 0)),
        _ => {}
    }
    let Some(n) = s.find('n') else {
        return s.parse::<i32>().ok().map(|b| (0, b));
    };
    let a = match &s[..n] {
        "" | "+" => 1,
        "-" => -1,
        t => t.parse::<i32>().ok()?,
    };
    let b = match &s[n + 1..] {
        "" => 0,
        t if t.starts_with('+') => t[1..].parse::<i32>().ok()?,
        t if t.starts_with('-') => -t[1..].parse::<i32>().ok()?,
        _ => return None,
    };
    Some((a, b))
}
