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

use crate::sidecar;

fn rec(k: &str, v: &str) -> (alloc::string::String, alloc::string::String) {
    (alloc::string::String::from(k), alloc::string::String::from(v))
}

#[test]
fn sidecar_round_trips() {
    let input = alloc::vec![rec("/a", "red,blue"), rec("/b", "green")];
    let decoded = sidecar::decode(&sidecar::encode(&input));
    assert_eq!(decoded, input);
}

#[test]
fn sidecar_encode_sorts_by_key() {
    let input = alloc::vec![rec("/z", "1"), rec("/a", "2")];
    let decoded = sidecar::decode(&sidecar::encode(&input));
    assert_eq!(decoded, alloc::vec![rec("/a", "2"), rec("/z", "1")]);
}

#[test]
fn sidecar_empty_round_trips() {
    assert!(sidecar::decode(&sidecar::encode(&[])).is_empty());
}

#[test]
fn sidecar_rejects_a_bad_magic_without_erroring() {
    let mut buf = sidecar::encode(&alloc::vec![rec("/a", "1")]);
    buf[0] ^= 0xFF;
    assert!(sidecar::decode(&buf).is_empty());
}

#[test]
fn sidecar_degrades_on_a_forward_version() {
    let mut buf = sidecar::encode(&alloc::vec![rec("/a", "1")]);
    buf[4] = 99;
    assert!(sidecar::decode(&buf).is_empty());
}

#[test]
fn sidecar_truncated_input_yields_nothing() {
    let buf = sidecar::encode(&alloc::vec![rec("/a", "1")]);
    assert!(sidecar::decode(&buf[..buf.len() - 1]).is_empty());
    assert!(sidecar::decode(&[]).is_empty());
}
