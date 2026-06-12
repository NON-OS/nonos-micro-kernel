// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use uefi::prelude::*;

pub fn print_u64(st: &mut SystemTable<Boot>, mut n: u64) {
    if n == 0 {
        let _ = st.stdout().output_string(cstr16!("0"));
        return;
    }
    let mut buf = [0u16; 20];
    let mut i = 19;
    while n > 0 && i > 0 {
        buf[i] = b'0' as u16 + (n % 10) as u16;
        n /= 10;
        i -= 1;
    }
    for c in &buf[i + 1..20] {
        let s = [*c, 0];
        let cs = unsafe { uefi::CStr16::from_u16_with_nul_unchecked(&s) };
        let _ = st.stdout().output_string(cs);
    }
}
