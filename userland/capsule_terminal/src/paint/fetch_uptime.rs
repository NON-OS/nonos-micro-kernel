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

use crate::term::util::{copy_into, format_u64};

pub fn uptime_str(elapsed_ms: u64, buf: &mut [u8]) -> usize {
    let total = elapsed_ms / 1000;
    let mut k = 0;
    k += format_u64(total / 60, &mut buf[k..]);
    k += copy_into(&mut buf[k..], b"m ");
    k += format_u64(total % 60, &mut buf[k..]);
    k += copy_into(&mut buf[k..], b"s");
    k
}
