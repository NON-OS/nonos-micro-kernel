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

//! Machine figures as text: rates, large counts and load averages. All of
//! it is integer arithmetic into a caller's buffer, like `format.rs`.

use super::format::{u32_decimal, u64_decimal};

fn push(out: &mut [u8], at: usize, bytes: &[u8]) -> usize {
    let end = (at + bytes.len()).min(out.len());
    out[at..end].copy_from_slice(&bytes[..end - at]);
    end
}

/// "0", "842", "1.2k", "35k": a per-second rate at the precision the eye
/// can use across a column of them.
pub fn rate_human(per_second: u32, out: &mut [u8]) -> usize {
    if per_second < 1000 {
        return u32_decimal(per_second, out);
    }
    if per_second < 10_000 {
        let n = u32_decimal(per_second / 1000, out);
        let n = push(out, n, b".");
        let n = u32_decimal(per_second % 1000 / 100, &mut out[n..]) + n;
        return push(out, n, b"k");
    }
    let n = u32_decimal(per_second / 1000, out);
    push(out, n, b"k")
}

/// A cumulative count: whole below ten thousand, then "12k", "3.4M".
pub fn count_human(count: u64, out: &mut [u8]) -> usize {
    if count < 10_000 {
        return u64_decimal(count, out);
    }
    if count < 1_000_000 {
        let n = u64_decimal(count / 1000, out);
        return push(out, n, b"k");
    }
    let n = u64_decimal(count / 1_000_000, out);
    let n = push(out, n, b".");
    let n = u64_decimal(count % 1_000_000 / 100_000, &mut out[n..]) + n;
    push(out, n, b"M")
}

/// A Q11 load average as "0.42": 2048 reads as 1.00.
pub fn load_human(q11: u64, out: &mut [u8]) -> usize {
    let hundredths = q11.saturating_mul(100) / 2048;
    let n = u64_decimal(hundredths / 100, out);
    let n = push(out, n, b".");
    let frac = (hundredths % 100) as u32;
    let n = if frac < 10 { push(out, n, b"0") } else { n };
    u32_decimal(frac, &mut out[n..]) + n
}
