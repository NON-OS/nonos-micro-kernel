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

use super::hex::parse_hex;
use super::hsl_fn::parse_hsl;
use super::named::named;
use super::rgb_fn::parse_rgb;

pub fn parse_color(v: &str) -> Option<u32> {
    let s = v.trim();
    if let Some(hex) = s.strip_prefix('#') {
        return parse_hex(hex);
    }
    let lower = s.to_ascii_lowercase();
    if lower.starts_with("rgb") {
        return parse_rgb(&lower);
    }
    if lower.starts_with("hsl") {
        return parse_hsl(&lower);
    }
    // currentColor keeps the inherited value: reporting None leaves the slot
    // untouched, which is what borders/text want.
    if lower == "currentcolor" {
        return None;
    }
    named(s)
}
