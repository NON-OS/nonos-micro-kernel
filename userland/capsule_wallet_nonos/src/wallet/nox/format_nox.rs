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

const WEI_PER_NOX: u128 = 1_000_000_000_000_000_000;

// Format a NOX amount (18-decimal wei) as whole units with two decimal places,
// e.g. 1_500_000_000_000_000_000 -> "1.50". Returns the byte length written.
pub fn format_nox(wei: u128, out: &mut [u8]) -> usize {
    let whole = wei / WEI_PER_NOX;
    let cents = (wei % WEI_PER_NOX) / (WEI_PER_NOX / 100);
    let mut n = write_u128(whole, out);
    if n + 3 > out.len() {
        return n;
    }
    out[n] = b'.';
    out[n + 1] = b'0' + (cents / 10) as u8;
    out[n + 2] = b'0' + (cents % 10) as u8;
    n += 3;
    n
}

fn write_u128(mut v: u128, out: &mut [u8]) -> usize {
    let mut tmp = [0u8; 40];
    let mut i = 0;
    loop {
        tmp[i] = b'0' + (v % 10) as u8;
        v /= 10;
        i += 1;
        if v == 0 {
            break;
        }
    }
    let n = i.min(out.len());
    for j in 0..n {
        out[j] = tmp[i - 1 - j];
    }
    n
}
