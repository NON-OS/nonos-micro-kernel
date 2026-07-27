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

use nonos_libc::mk_debug;

use crate::error::{exit_code, HdaError};

const PREFIX: &[u8] = b"[HDA] setup-fail code=";
const RC_TAG: &[u8] = b" rc=";

fn put_str(buf: &mut [u8], n: usize, s: &[u8]) -> usize {
    buf[n..n + s.len()].copy_from_slice(s);
    n + s.len()
}

fn put_u64(buf: &mut [u8], n: usize, v: u64) -> usize {
    let mut digits = [0u8; 20];
    let mut count = 0usize;
    let mut rest = v;
    loop {
        digits[count] = b'0' + (rest % 10) as u8;
        count += 1;
        rest /= 10;
        if rest == 0 {
            break;
        }
    }
    let mut w = n;
    while count > 0 {
        count -= 1;
        buf[w] = digits[count];
        w += 1;
    }
    w
}

pub fn mark_setup_fail(e: HdaError) {
    let mut buf = [0u8; 64];
    let mut n = put_str(&mut buf, 0, PREFIX);
    n = put_u64(&mut buf, n, exit_code(e) as u64);
    if let HdaError::BrokerCallFailed(rc) = e {
        n = put_str(&mut buf, n, RC_TAG);
        if rc < 0 {
            buf[n] = b'-';
            n += 1;
        }
        n = put_u64(&mut buf, n, rc.unsigned_abs());
    }
    buf[n] = b'\n';
    mk_debug(buf.as_ptr(), n + 1);
}
