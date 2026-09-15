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

//! Each command against the byte layout in Part 3 of the specification.

use super::consts::{DIGEST_LEN, NONCE_LEN};
use super::create::build_create;
use super::flush::build_flush;
use super::hmac::build_hmac;
use super::pcrs::{bitmap, BOUND_PCRS};
use super::policy::{build_get_digest, build_policy_pcr};
use super::session::build_start;

fn size_of(cmd: &[u8]) -> usize {
    u32::from_be_bytes([cmd[2], cmd[3], cmd[4], cmd[5]]) as usize
}

#[test]
fn start_session_is_unbound_unsalted_policy_sha256() {
    let nonce = [0x11u8; NONCE_LEN];
    let cmd = build_start(&nonce);
    assert_eq!(size_of(&cmd), cmd.len());
    assert_eq!(&cmd[..2], &[0x80, 0x01]);
    assert_eq!(&cmd[6..10], &[0, 0, 0x01, 0x76]);
    assert_eq!(&cmd[10..14], &[0x40, 0, 0, 0x07], "tpmKey null");
    assert_eq!(&cmd[14..18], &[0x40, 0, 0, 0x07], "bind null");
    assert_eq!(&cmd[18..20], &[0, NONCE_LEN as u8]);
    assert_eq!(&cmd[20..36], &nonce);
    assert_eq!(&cmd[36..38], &[0, 0], "no salt");
    assert_eq!(cmd[38], 0x01, "TPM_SE_POLICY");
    assert_eq!(&cmd[39..41], &[0, 0x10], "symmetric null");
    assert_eq!(&cmd[41..43], &[0, 0x0B], "sha256");
    assert_eq!(cmd.len(), 43);
}

#[test]
fn bound_pcrs_map_to_the_right_bits() {
    assert_eq!(BOUND_PCRS, [0, 4, 7, 9]);
    assert_eq!(bitmap(&BOUND_PCRS), [0b1001_0001, 0b0000_0010, 0]);
    assert_eq!(bitmap(&[23]), [0, 0, 0x80]);
    assert_eq!(bitmap(&[24]), [0, 0, 0], "out of the bank is dropped, not wrapped");
}

#[test]
fn policy_pcr_asserts_no_value_and_selects_the_sha256_bank() {
    let cmd = build_policy_pcr(0x0300_0000, &BOUND_PCRS);
    assert_eq!(size_of(&cmd), cmd.len());
    assert_eq!(&cmd[6..10], &[0, 0, 0x01, 0x7F]);
    assert_eq!(&cmd[10..14], &[0x03, 0, 0, 0]);
    assert_eq!(&cmd[14..16], &[0, 0], "empty pcrDigest");
    assert_eq!(&cmd[16..20], &[0, 0, 0, 1], "one selection");
    assert_eq!(&cmd[20..22], &[0, 0x0B]);
    assert_eq!(cmd[22], 3);
    assert_eq!(&cmd[23..26], &bitmap(&BOUND_PCRS));
    assert_eq!(cmd.len(), 26);
}

#[test]
fn get_digest_carries_only_the_session() {
    let cmd = build_get_digest(0x0300_0001);
    assert_eq!(cmd, [0x80, 0x01, 0, 0, 0, 14, 0, 0, 0x01, 0x89, 0x03, 0, 0, 0x01]);
}

#[test]
fn create_primary_binds_the_policy_into_the_template() {
    let policy = [0x5Au8; DIGEST_LEN];
    let cmd = build_create(&policy);
    assert_eq!(size_of(&cmd), cmd.len());
    assert_eq!(&cmd[..2], &[0x80, 0x02], "sessions tag");
    assert_eq!(&cmd[6..10], &[0, 0, 0x01, 0x31]);
    assert_eq!(&cmd[10..14], &[0x40, 0, 0, 0x01], "owner hierarchy");
    assert_eq!(&cmd[14..18], &[0, 0, 0, 9], "auth area size");
    assert_eq!(&cmd[18..22], &[0x40, 0, 0, 0x09], "password session");
    assert_eq!(&cmd[27..33], &[0, 4, 0, 0, 0, 0], "empty sensitive");
    let public_len = u16::from_be_bytes([cmd[33], cmd[34]]) as usize;
    let public = &cmd[35..35 + public_len];
    assert_eq!(&public[..2], &[0, 0x08], "keyedhash");
    assert_eq!(&public[2..4], &[0, 0x0B], "name alg sha256");
    assert_eq!(&public[4..8], &[0, 0x04, 0, 0x32], "no userWithAuth");
    assert_eq!(&public[8..10], &[0, 32]);
    assert_eq!(&public[10..42], &policy);
    assert_eq!(&public[42..46], &[0, 0x05, 0, 0x0B], "hmac sha256");
    assert_eq!(&public[46..48], &[0, 0], "unique empty");
    assert_eq!(public_len, 48);
    assert_eq!(&cmd[35 + public_len..], &[0, 0, 0, 0, 0, 0], "no outside info, no creation pcrs");
}

#[test]
fn hmac_is_authorised_by_the_session_with_an_empty_hmac() {
    let cmd = build_hmac(0x8000_0002, 0x0300_0000, b"nonos.volume.v1");
    assert_eq!(size_of(&cmd), cmd.len());
    assert_eq!(&cmd[..2], &[0x80, 0x02]);
    assert_eq!(&cmd[6..10], &[0, 0, 0x01, 0x55]);
    assert_eq!(&cmd[10..14], &[0x80, 0, 0, 0x02], "key handle");
    assert_eq!(&cmd[14..18], &[0, 0, 0, 9]);
    assert_eq!(&cmd[18..22], &[0x03, 0, 0, 0], "the policy session, not the password");
    assert_eq!(&cmd[22..27], &[0, 0, 0, 0, 0], "no nonce, no attributes, no hmac");
    assert_eq!(&cmd[27..29], &[0, 15]);
    assert_eq!(&cmd[29..44], b"nonos.volume.v1");
    assert_eq!(&cmd[44..46], &[0, 0x0B]);
    assert_eq!(cmd.len(), 46);
}

#[test]
fn flush_names_the_handle_as_a_parameter() {
    assert_eq!(
        build_flush(0x0300_0000),
        [0x80, 0x01, 0, 0, 0, 14, 0, 0, 0x01, 0x65, 0x03, 0, 0, 0]
    );
}
