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

pub fn u64_decimal(mut value: u64, dst: &mut [u8; 20]) -> &[u8] {
    if value == 0 {
        dst[0] = b'0';
        return &dst[..1];
    }
    let mut tmp = [0u8; 20];
    let mut idx = 0;
    while value > 0 {
        tmp[idx] = b'0' + (value % 10) as u8;
        value /= 10;
        idx += 1;
    }
    let mut out_len = 0;
    while idx > 0 {
        idx -= 1;
        dst[out_len] = tmp[idx];
        out_len += 1;
    }
    &dst[..out_len]
}


// `VERSION` is the repository file verbatim, so it carries the newline the file
// ends with. Every surface that prints it wants the token, not the line.
pub fn trimmed(bytes: &[u8]) -> &[u8] {
    let mut end = bytes.len();
    while end > 0 && (bytes[end - 1] == b'\n' || bytes[end - 1] == b'\r' || bytes[end - 1] == b' ') {
        end -= 1;
    }
    &bytes[..end]
}

fn push(dst: &mut [u8], at: usize, src: &[u8]) -> usize {
    let mut n = at;
    for byte in src {
        if n < dst.len() {
            dst[n] = *byte;
            n += 1;
        }
    }
    n
}

fn push_u64(dst: &mut [u8], at: usize, value: u64) -> usize {
    let mut tmp = [0u8; 20];
    push(dst, at, u64_decimal(value, &mut tmp))
}

// Q11 fixed point: the kernel publishes 2048 for a load of 1.00 and no float
// crosses the ABI. Hundredths round half away from zero, the same rule the
// terminal rail uses, so 2048 reads "1.00" and 860 reads "0.42".
pub fn load_q11(q: u64, dst: &mut [u8; 12]) -> &[u8] {
    let hundredths = q.saturating_mul(100).saturating_add(1024) / 2048;
    let mut n = push_u64(dst, 0, hundredths / 100);
    n = push(dst, n, b".");
    if hundredths % 100 < 10 {
        n = push(dst, n, b"0");
    }
    let n = push_u64(dst, n, hundredths % 100);
    &dst[..n]
}

// Kibibytes scaled to the largest unit that keeps the figure short: whole KiB
// below a mebibyte, then one decimal place for MiB and GiB.
pub fn kib(kb: u64, dst: &mut [u8; 24]) -> &[u8] {
    if kb < 1024 {
        let n = push_u64(dst, 0, kb);
        let n = push(dst, n, b" KiB");
        return &dst[..n];
    }
    let (tenths, unit): (u64, &[u8]) = if kb < 1024 * 1024 {
        (kb.saturating_mul(10) / 1024, b" MiB")
    } else {
        (kb.saturating_mul(10) / (1024 * 1024), b" GiB")
    };
    let mut n = push_u64(dst, 0, tenths / 10);
    n = push(dst, n, b".");
    n = push_u64(dst, n, tenths % 10);
    let n = push(dst, n, unit);
    &dst[..n]
}

pub fn hex_u64(v: u64, dst: &mut [u8; 20]) -> &[u8] {
    dst[0] = b'0';
    dst[1] = b'x';
    let mut n = 2;
    let mut shift = 60i32;
    while shift >= 0 {
        let nibble = ((v >> shift) & 0xf) as u8;
        if nibble != 0 || n > 2 || shift == 0 {
            dst[n] = if nibble < 10 { b'0' + nibble } else { b'a' + nibble - 10 };
            n += 1;
        }
        shift -= 4;
    }
    &dst[..n]
}
