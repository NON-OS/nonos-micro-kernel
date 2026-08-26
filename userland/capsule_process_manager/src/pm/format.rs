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

pub fn u32_decimal(mut value: u32, out: &mut [u8]) -> usize {
    if value == 0 {
        if out.is_empty() {
            return 0;
        }
        out[0] = b'0';
        return 1;
    }
    let mut tmp = [0u8; 10];
    let mut len = 0;
    while value > 0 && len < tmp.len() {
        tmp[len] = b'0' + (value % 10) as u8;
        value /= 10;
        len += 1;
    }
    let n = len.min(out.len());
    for i in 0..n {
        out[i] = tmp[len - 1 - i];
    }
    n
}

// Widen a count the kernel reports as 64-bit. Delegates below 2^32 so every
// number in the UI comes out of one code path.
pub fn u64_decimal(value: u64, out: &mut [u8]) -> usize {
    if value <= u32::MAX as u64 {
        return u32_decimal(value as u32, out);
    }
    let mut buf = [0u8; 20];
    let mut i = buf.len();
    let mut v = value;
    while v > 0 {
        i -= 1;
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    let n = (buf.len() - i).min(out.len());
    out[..n].copy_from_slice(&buf[i..i + n]);
    n
}

// One decimal place, e.g. "11.0%". cpu_pct is a whole percent from the kernel,
// so the tenth is always 0 today; the width is what keeps a column of numbers
// aligned when a future sample carries one.
pub fn pct_1dp(pct: u8, out: &mut [u8]) -> usize {
    let mut n = u32_decimal(pct as u32, out);
    if n + 3 <= out.len() {
        out[n] = b'.';
        out[n + 1] = b'0';
        out[n + 2] = b'%';
        n += 3;
    }
    n
}

// Scheduler state names, indexed by the kernel state code. A capsule blocked on
// IPC is parked by the scheduler; it is alive and answers the instant a message
// arrives, so "idle" reads truer than "sleeping" or "waiting".
pub fn state_label(state: u8) -> &'static [u8] {
    match state {
        0 => b"new",
        1 => b"ready",
        2 => b"running",
        3 => b"idle",
        4 => b"stopped",
        5 => b"zombie",
        _ => b"exited",
    }
}

// Uptime as a compact, always-incrementing label: "45s", then "3m05s",
// "1h02m", "2d03h". It changes every refresh for every process, so the table
// reads as live even when a process is sitting idle.
pub fn uptime_human(secs: u64, out: &mut [u8]) -> usize {
    if secs < 60 {
        let n = put(out, 0, secs as u32, false);
        put_unit(out, n, b's') + n
    } else if secs < 3600 {
        let mut n = put(out, 0, (secs / 60) as u32, false);
        n += put_unit(out, n, b'm');
        n += put(out, n, (secs % 60) as u32, true);
        n + put_unit(out, n, b's')
    } else if secs < 86400 {
        let mut n = put(out, 0, (secs / 3600) as u32, false);
        n += put_unit(out, n, b'h');
        n += put(out, n, ((secs % 3600) / 60) as u32, true);
        n + put_unit(out, n, b'm')
    } else {
        let mut n = put(out, 0, (secs / 86400) as u32, false);
        n += put_unit(out, n, b'd');
        n += put(out, n, ((secs % 86400) / 3600) as u32, true);
        n + put_unit(out, n, b'h')
    }
}

// Write `v` at `out[at..]`, optionally zero-padded to two digits; returns bytes
// written.
fn put(out: &mut [u8], at: usize, v: u32, pad2: bool) -> usize {
    let mut used = 0;
    if pad2 && v < 10 && at < out.len() {
        out[at] = b'0';
        used = 1;
    }
    let start = (at + used).min(out.len());
    used + u32_decimal(v, &mut out[start..])
}

fn put_unit(out: &mut [u8], at: usize, unit: u8) -> usize {
    if at < out.len() {
        out[at] = unit;
        1
    } else {
        0
    }
}

// Resident memory as a compact "N.N MB" above a megabyte, else "N KB".
pub fn mem_human(kb: u64, out: &mut [u8]) -> usize {
    if kb >= 1024 {
        let mb = kb / 1024;
        let tenths = (kb % 1024) * 10 / 1024;
        let mut n = u32_decimal(mb as u32, out);
        if n + 4 <= out.len() {
            out[n] = b'.';
            out[n + 1] = b'0' + tenths as u8;
            out[n + 2] = b' ';
            out[n + 3] = b'M';
            n += 4;
            if n < out.len() {
                out[n] = b'B';
                n += 1;
            }
        }
        n
    } else {
        let mut n = u32_decimal(kb as u32, out);
        for &c in b" KB" {
            if n < out.len() {
                out[n] = c;
                n += 1;
            }
        }
        n
    }
}

// Capability bits and their short names, in enum order, so the selected
// process can show exactly which authorities it was granted.
pub const CAP_TABLE: &[(u64, &[u8])] = &[
    (1 << 0, b"exec"),
    (1 << 1, b"io"),
    (1 << 2, b"net"),
    (1 << 3, b"ipc"),
    (1 << 4, b"mem"),
    (1 << 5, b"crypto"),
    (1 << 6, b"fs"),
    (1 << 7, b"hw"),
    (1 << 8, b"debug"),
    (1 << 9, b"admin"),
    (1 << 10, b"regsvc"),
    (1 << 11, b"gquery"),
    (1 << 12, b"gcreate"),
    (1 << 13, b"gmap"),
    (1 << 14, b"gpresent"),
    (1 << 15, b"devenum"),
    (1 << 16, b"driver"),
    (1 << 17, b"mmio"),
    (1 << 18, b"irq"),
    (1 << 19, b"dma"),
    (1 << 20, b"pio"),
    (1 << 21, b"input"),
    (1 << 22, b"time"),
    (1 << 23, b"spawnbroker"),
    (1 << 24, b"spawnwindow"),
    (1 << 25, b"procctl"),
];

// "Ndrop 0xHEX": how many capabilities are granted, and the raw bit set, so the
// authority of every process is visible at a glance.
pub fn caps_summary(caps: u64, out: &mut [u8]) -> usize {
    let mut n = u32_decimal(caps.count_ones(), out);
    for &c in b"  0x" {
        if n < out.len() {
            out[n] = c;
            n += 1;
        }
    }
    let mut started = false;
    for shift in (0..64).step_by(4).rev() {
        let nyb = ((caps >> shift) & 0xf) as u8;
        if nyb != 0 || started || shift == 0 {
            started = true;
            if n < out.len() {
                out[n] = if nyb < 10 { b'0' + nyb } else { b'a' + (nyb - 10) };
                n += 1;
            }
        }
    }
    n
}
