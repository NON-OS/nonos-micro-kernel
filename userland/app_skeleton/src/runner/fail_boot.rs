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

use super::fail::fail;

pub(super) fn fail_boot(code: i32, title: &[u8], reason: &str) -> ! {
    let prefix = b"[app] boot fail: ";
    let mut buf = [0u8; 144];
    buf[..prefix.len()].copy_from_slice(prefix);
    let mut len = prefix.len();
    let n = core::cmp::min(title.len(), buf.len() - len - 3);
    buf[len..len + n].copy_from_slice(&title[..n]);
    len += n;
    if len + 2 < buf.len() {
        buf[len..len + 2].copy_from_slice(b": ");
        len += 2;
    }
    let reason_bytes = reason.as_bytes();
    let m = core::cmp::min(reason_bytes.len(), buf.len() - len - 1);
    buf[len..len + m].copy_from_slice(&reason_bytes[..m]);
    len += m;
    buf[len] = b'\n';
    fail(code, &buf[..len + 1])
}
