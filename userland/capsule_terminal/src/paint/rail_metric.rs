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
use crate::rail::value::Metric;
use crate::term::util::copy_into;

/// What a figure with no source reads as. Every non-`Known` metric renders as
/// this, because a zero would claim a measurement NONOS never took.
pub const DASH: &str = "—";

pub fn one<'a, T: Copy>(buf: &'a mut [u8], m: Metric<T>, f: fn(&mut [u8], T) -> usize) -> &'a str {
    let n = write(buf, m, f);
    core::str::from_utf8(&buf[..n]).unwrap_or(DASH)
}

/// Two figures of the same kind side by side, as the memory and rate rows want
/// them: either half is independently a dash.
pub fn pair<'a, T: Copy>(
    buf: &'a mut [u8],
    a: Metric<T>,
    b: Metric<T>,
    f: fn(&mut [u8], T) -> usize,
) -> &'a str {
    let mut n = write(buf, a, f);
    n += copy_into(&mut buf[n..], b" / ");
    n += write(&mut buf[n..], b, f);
    core::str::from_utf8(&buf[..n]).unwrap_or(DASH)
}

fn write<T: Copy>(buf: &mut [u8], m: Metric<T>, f: fn(&mut [u8], T) -> usize) -> usize {
    match m.value() {
        Some(v) => f(buf, v),
        None => copy_into(buf, DASH.as_bytes()),
    }
}
