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

use alloc::vec::Vec;
use nonos_base64::encode;

use super::args::PullArgs;

pub fn extra_headers(a: &PullArgs) -> Vec<u8> {
    let mut e = Vec::new();
    if let Some(cred) = &a.auth {
        e.extend_from_slice(b"Authorization: Basic ");
        e.extend_from_slice(&encode(cred));
        e.extend_from_slice(b"\r\n");
    }
    for h in &a.headers {
        e.extend_from_slice(h);
        e.extend_from_slice(b"\r\n");
    }
    e
}
