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

//! The `-l` timestamp column. The vfs reports an mtime in milliseconds since
//! the epoch and nothing else, so the stamp is rendered here rather than asked
//! of libc, which only knows the current civil time.

use alloc::vec::Vec;

const MONTHS: [&[u8; 3]; 12] = [
    b"Jan", b"Feb", b"Mar", b"Apr", b"May", b"Jun", b"Jul", b"Aug", b"Sep", b"Oct", b"Nov", b"Dec",
];

/// `Mon DD HH:MM`, always twelve bytes so the name column never shifts. An
/// mtime of zero means the vfs has no date for the entry, and gets a dash of
/// the same width rather than a fabricated one.
pub fn stamp(mtime_ms: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(12);
    if mtime_ms == 0 {
        out.extend_from_slice(b"           -");
        return out;
    }
    let days = mtime_ms / 86_400_000;
    let secs = mtime_ms % 86_400_000 / 1000;
    let (month, day) = civil(days);
    out.extend_from_slice(MONTHS[month as usize - 1]);
    out.push(b' ');
    out.push(if day >= 10 { b'0' + (day / 10) as u8 } else { b' ' });
    out.push(b'0' + (day % 10) as u8);
    out.push(b' ');
    two(&mut out, secs / 3600);
    out.push(b':');
    two(&mut out, secs % 3600 / 60);
    out
}

fn two(out: &mut Vec<u8>, value: u64) {
    out.push(b'0' + (value / 10) as u8);
    out.push(b'0' + (value % 10) as u8);
}

/// Howard Hinnant's era-based civil-from-days, shifted so the era count stays
/// non-negative for every epoch day this system can produce. Integer only.
fn civil(days: u64) -> (u64, u64) {
    let z = days + 719_468;
    let era = z / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    (month, day)
}
