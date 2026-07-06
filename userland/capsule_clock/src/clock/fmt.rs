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

pub fn two(v: u8) -> [u8; 2] {
    [b'0' + (v / 10) % 10, b'0' + v % 10]
}

pub fn hms(h: u8, m: u8, s: u8) -> [u8; 8] {
    let hh = two(h);
    let mm = two(m);
    let ss = two(s);
    [hh[0], hh[1], b':', mm[0], mm[1], b':', ss[0], ss[1]]
}

pub fn ms_clock(ms: u64) -> [u8; 8] {
    let cs = (ms / 10 % 100) as u8;
    let s = (ms / 1000 % 60) as u8;
    let m = (ms / 60000 % 100) as u8;
    let mm = two(m);
    let ss = two(s);
    let cc = two(cs);
    [mm[0], mm[1], b':', ss[0], ss[1], b'.', cc[0], cc[1]]
}

pub fn year4(y: u16) -> [u8; 4] {
    [
        b'0' + ((y / 1000) % 10) as u8,
        b'0' + ((y / 100) % 10) as u8,
        b'0' + ((y / 10) % 10) as u8,
        b'0' + (y % 10) as u8,
    ]
}

pub fn weekday(y: u16, m: u8, d: u8) -> &'static str {
    let t = [0i32, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut yy = y as i32;
    if m < 3 {
        yy -= 1;
    }
    let mi = (m as usize).saturating_sub(1).min(11);
    let idx = ((yy + yy / 4 - yy / 100 + yy / 400 + t[mi] + d as i32).rem_euclid(7)) as usize;
    [
        "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday",
    ][idx.min(6)]
}

pub fn month_name(m: u8) -> &'static str {
    [
        "---", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ][(m as usize).min(12)]
}
