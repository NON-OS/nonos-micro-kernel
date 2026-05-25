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

const PREFIX: &[u8] = b"[virtio_rng] ";
const NEWLINE: &[u8] = b"\n";
const MAX_LABEL: usize = 200;

pub fn marker(label: &[u8]) {
    let label_len = if label.len() > MAX_LABEL { MAX_LABEL } else { label.len() };
    let _ = mk_debug(PREFIX.as_ptr(), PREFIX.len());
    let _ = mk_debug(label.as_ptr(), label_len);
    let _ = mk_debug(NEWLINE.as_ptr(), NEWLINE.len());
}
