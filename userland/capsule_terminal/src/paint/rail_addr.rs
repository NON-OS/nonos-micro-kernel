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
use super::rail_metric::{one, DASH};
use crate::rail::value::Metric;
use crate::term::util::{copy_into, format_u64};

const HEX: &[u8; 16] = b"0123456789abcdef";

pub fn ipv4_into(buf: &mut [u8], ip: [u8; 4]) -> usize {
    let mut n = 0;
    for (i, o) in ip.iter().enumerate() {
        if i > 0 {
            n += copy_into(&mut buf[n..], b".");
        }
        n += format_u64(*o as u64, &mut buf[n..]);
    }
    n
}

/// Full eight-group form. NONOS runs no v6 stack, so this exists to render an
/// address correctly if one ever arrives rather than to shorten one that has.
pub fn ipv6_into(buf: &mut [u8], ip: [u8; 16]) -> usize {
    let mut n = 0;
    for g in 0..8 {
        if g > 0 {
            n += copy_into(&mut buf[n..], b":");
        }
        let v = ((ip[g * 2] as u16) << 8) | ip[g * 2 + 1] as u16;
        for s in [12, 8, 4, 0] {
            n += copy_into(&mut buf[n..], &[HEX[((v >> s) & 0xF) as usize]]);
        }
    }
    n
}

/// The address with its prefix length. A lease with an address but no prefix is
/// still a real address, so the suffix is dropped rather than the whole row.
pub fn ipv4_pfx<'a>(buf: &'a mut [u8], ip: Metric<[u8; 4]>, pfx: Metric<u8>) -> &'a str {
    let Some(addr) = ip.value() else {
        return DASH;
    };
    let mut n = ipv4_into(buf, addr);
    if let Some(bits) = pfx.value() {
        n += copy_into(&mut buf[n..], b"/");
        n += format_u64(bits as u64, &mut buf[n..]);
    }
    core::str::from_utf8(&buf[..n]).unwrap_or(DASH)
}

pub fn ipv6_str<'a>(buf: &'a mut [u8], ip: Metric<[u8; 16]>) -> &'a str {
    one(buf, ip, ipv6_into)
}
