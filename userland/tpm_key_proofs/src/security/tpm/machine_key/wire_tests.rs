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

//! Framing and the two reads every response goes through.

use super::consts::HEADER_LEN;
use super::error::KeyError;
use super::wire::{checked, digest_at, frame, u32_at};
use crate::security::tpm::error::TpmError;

#[test]
fn frame_size_counts_the_header() {
    let cmd = frame(0x8001, 0x0000_0165, &[0xAA; 4]);
    assert_eq!(cmd.len(), HEADER_LEN + 4);
    assert_eq!(&cmd[..2], &[0x80, 0x01]);
    assert_eq!(u32::from_be_bytes([cmd[2], cmd[3], cmd[4], cmd[5]]), 14);
    assert_eq!(&cmd[6..10], &[0, 0, 0x01, 0x65]);
    assert_eq!(&cmd[10..], &[0xAA; 4]);
}

#[test]
fn success_passes_the_response_through() {
    let resp = [0x80, 0x01, 0, 0, 0, 10, 0, 0, 0, 0];
    assert_eq!(checked(&resp).unwrap().len(), 10);
}

#[test]
fn a_response_code_is_carried_whole() {
    // TPM_RC_POLICY_FAIL with parameter number 1: 0x0000099D.
    let resp = [0x80, 0x01, 0, 0, 0, 10, 0, 0, 0x09, 0x9D];
    assert_eq!(checked(&resp), Err(KeyError::Refused(0x99D)));
}

#[test]
fn a_short_header_is_a_transport_fault_not_a_refusal() {
    assert_eq!(checked(&[0x80, 0x01, 0]), Err(KeyError::Tpm(TpmError::InvalidResponse)));
}

#[test]
fn u32_at_refuses_to_read_past_the_end() {
    assert_eq!(u32_at(&[1, 2, 3], 0), Err(KeyError::Tpm(TpmError::InvalidResponse)));
    assert_eq!(u32_at(&[1, 2, 3, 4], 0), Ok(0x0102_0304));
}

#[test]
fn digest_must_be_exactly_sha256_sized() {
    let mut ok = vec![0, 32];
    ok.extend_from_slice(&[7u8; 32]);
    assert_eq!(digest_at(&ok, 0), Ok([7u8; 32]));
    let mut short = vec![0, 20];
    short.extend_from_slice(&[7u8; 20]);
    assert!(digest_at(&short, 0).is_err());
    let truncated = &ok[..30];
    assert!(digest_at(truncated, 0).is_err());
}
