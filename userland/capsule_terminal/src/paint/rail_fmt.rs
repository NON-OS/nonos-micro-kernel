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

use crate::term::util::{copy_into, format_u64};

pub fn num(buf: &mut [u8], v: u64) -> &str {
    let n = format_u64(v, buf);
    core::str::from_utf8(&buf[..n]).unwrap_or("")
}

pub fn pct(buf: &mut [u8], v: u32) -> &str {
    let mut n = format_u64(v as u64, buf);
    n += copy_into(&mut buf[n..], b"%");
    core::str::from_utf8(&buf[..n]).unwrap_or("")
}

/// Resident kilobytes as mebibytes with one decimal, so a rail column shows a
/// number that moves without ever widening past four glyphs plus its unit.
pub fn mib(buf: &mut [u8], kb: u64) -> &str {
    let tenths = kb.saturating_mul(10) / 1024;
    let mut n = format_u64(tenths / 10, buf);
    n += copy_into(&mut buf[n..], b".");
    n += format_u64(tenths % 10, &mut buf[n..]);
    n += copy_into(&mut buf[n..], b" MB");
    core::str::from_utf8(&buf[..n]).unwrap_or("")
}

pub fn uptime(buf: &mut [u8], ms: u64) -> &str {
    let s = ms / 1000;
    let mut n = format_u64(s / 3600, buf);
    n += copy_into(&mut buf[n..], b"h ");
    n += format_u64(s % 3600 / 60, &mut buf[n..]);
    n += copy_into(&mut buf[n..], b"m ");
    n += format_u64(s % 60, &mut buf[n..]);
    n += copy_into(&mut buf[n..], b"s");
    core::str::from_utf8(&buf[..n]).unwrap_or("")
}
