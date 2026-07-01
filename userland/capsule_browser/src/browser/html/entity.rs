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

pub fn push_decoded(out: &mut String, entity: &str) {
    let s = match entity {
        "amp" => "&",
        "lt" => "<",
        "gt" => ">",
        "quot" => "\"",
        "apos" => "'",
        "nbsp" => " ",
        "mdash" => "--",
        "ndash" | "minus" => "-",
        "hellip" => "...",
        "lsquo" | "rsquo" | "sbquo" => "'",
        "ldquo" | "rdquo" | "bdquo" => "\"",
        "copy" => "(c)",
        "reg" => "(r)",
        "trade" => "(tm)",
        "bull" | "middot" => "*",
        "deg" => "deg",
        "times" => "x",
        "laquo" => "<<",
        "raquo" => ">>",
        "frasl" => "/",
        _ => {
            if let Some(n) = entity.strip_prefix('#') {
                let cp = if let Some(h) = n.strip_prefix('x').or_else(|| n.strip_prefix('X')) {
                    u32::from_str_radix(h, 16).ok()
                } else {
                    n.parse::<u32>().ok()
                };
                if let Some(c) = cp.and_then(char::from_u32).filter(|c| !c.is_control()) {
                    out.push(c);
                    return;
                }
            }
            out.push('&');
            out.push_str(entity);
            out.push(';');
            return;
        }
    };
    out.push_str(s);
}
