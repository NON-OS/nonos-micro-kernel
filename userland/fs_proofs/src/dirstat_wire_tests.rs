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

// The OP_DIRSTAT handler resolves `super::util` and `super::path`, so it cannot
// be host-included; what these proofs pin is the reply-body layout, which is
// the part that breaks silently when either side of the wire is edited alone.

use crate::protocol::HDR_LEN;

fn dirstat_body(files: u32, dirs: u32, bytes: u64, truncated: bool) -> alloc::vec::Vec<u8> {
    let mut body = alloc::vec::Vec::with_capacity(20);
    body.extend_from_slice(&files.to_le_bytes());
    body.extend_from_slice(&dirs.to_le_bytes());
    body.extend_from_slice(&bytes.to_le_bytes());
    body.extend_from_slice(&(truncated as u32).to_le_bytes());
    body
}

#[test]
fn dirstat_reply_layout_round_trips() {
    let body = dirstat_body(2, 1, 10, true);
    assert_eq!(body.len(), 20);
    let rx = crate::protocol::encode_response(22, 0, 22, 0, &body);
    assert_eq!(u32::from_le_bytes(rx[HDR_LEN + 4..HDR_LEN + 8].try_into().unwrap()), 2);
    assert_eq!(u32::from_le_bytes(rx[HDR_LEN + 8..HDR_LEN + 12].try_into().unwrap()), 1);
    assert_eq!(u64::from_le_bytes(rx[HDR_LEN + 12..HDR_LEN + 20].try_into().unwrap()), 10);
    assert_eq!(u32::from_le_bytes(rx[HDR_LEN + 20..HDR_LEN + 24].try_into().unwrap()), 1);
    assert_eq!(rx.len(), HDR_LEN + 4 + 20);
}

#[test]
fn dirstat_opcode_is_the_next_free_value() {
    assert_eq!(crate::protocol::OP_DIRSTAT, 22);
}
