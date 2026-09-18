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

//! Small formatters the inspector and the memory screens share: names for
//! the kernel's codes, shares, a user/kernel split, a region count.

use super::format::{pct_1dp, u32_decimal};

/// The scheduling class the kernel reported: the `priority` byte of an entry.
pub fn priority_label(code: u8) -> &'static [u8] {
    match code {
        0 => b"idle",
        1 => b"low",
        2 => b"normal",
        3 => b"high",
        4 => b"realtime",
        _ => b"?",
    }
}

/// `kb` as a whole percentage of `total_kb`, clamped; zero of nothing is zero.
pub fn share_pct(kb: u64, total_kb: u64) -> u8 {
    if total_kb == 0 {
        0
    } else {
        (kb.saturating_mul(100) / total_kb).min(100) as u8
    }
}

/// "1.0% / 3.0%": the share that was the process's own code, then the
/// kernel's work for it.
pub fn split(user: u8, kernel: u8, out: &mut [u8]) -> usize {
    let mut n = pct_1dp(user, out);
    for &c in b" / " {
        if n < out.len() {
            out[n] = c;
            n += 1;
        }
    }
    n + pct_1dp(kernel, &mut out[n..])
}

/// " in 7 regions" after a size.
pub fn regions(count: u32, out: &mut [u8], at: usize) -> usize {
    let mut n = at;
    for &c in b" in " {
        if n < out.len() {
            out[n] = c;
            n += 1;
        }
    }
    n += u32_decimal(count, &mut out[n..]);
    let end = (n + b" regions".len()).min(out.len());
    out[n..end].copy_from_slice(&b" regions"[..end - n]);
    end
}
