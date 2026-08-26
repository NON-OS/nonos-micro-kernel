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

use alloc::format;
use alloc::string::{String, ToString};

const UNITS: [(u64, &str); 3] = [(1 << 30, "GB"), (1 << 20, "MB"), (1 << 10, "KB")];

pub fn duration(ms: i64) -> String {
    if ms <= 0 {
        return "--:--".to_string();
    }
    let secs = ms / 1000;
    let (h, m, s) = (secs / 3600, (secs % 3600) / 60, secs % 60);
    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        format!("{:02}:{:02}", m, s)
    }
}

pub fn size(bytes: u64) -> String {
    if bytes == 0 {
        return "--".to_string();
    }
    for (scale, unit) in UNITS {
        if bytes >= scale {
            let tenths = bytes * 10 / scale;
            return format!("{}.{} {}", tenths / 10, tenths % 10, unit);
        }
    }
    format!("{} B", bytes)
}

pub fn resolution(w: u32, h: u32) -> String {
    if w == 0 || h == 0 {
        return "--".to_string();
    }
    format!("{}x{}", w, h)
}

pub fn count(n: usize, one: &str, many: &str) -> String {
    if n == 1 {
        format!("1 {}", one)
    } else {
        format!("{} {}", n, many)
    }
}
