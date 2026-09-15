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

//! Responses, well formed and not.

use super::create::parse_create;
use super::error::KeyError;
use super::hmac::parse_hmac;
use super::policy::{parse_get_digest, parse_policy_pcr};
use super::session::parse_start;

fn header(tag: u16, rc: u32, body_len: usize) -> Vec<u8> {
    let mut v = tag.to_be_bytes().to_vec();
    v.extend_from_slice(&((10 + body_len) as u32).to_be_bytes());
    v.extend_from_slice(&rc.to_be_bytes());
    v
}

#[test]
fn session_handle_leads_the_response() {
    let mut r = header(0x8001, 0, 22);
    r.extend_from_slice(&[0x03, 0, 0, 0x02]);
    r.extend_from_slice(&[0, 16]);
    r.extend_from_slice(&[9u8; 16]);
    assert_eq!(parse_start(&r), Ok(0x0300_0002));
}

#[test]
fn a_refused_session_is_not_a_handle() {
    let r = header(0x8001, 0x0000_0902, 0);
    assert_eq!(parse_start(&r), Err(KeyError::Refused(0x902)));
}

#[test]
fn policy_pcr_answers_with_a_bare_header() {
    assert_eq!(parse_policy_pcr(&header(0x8001, 0, 0)), Ok(()));
    assert!(parse_policy_pcr(&header(0x8001, 0x1C4, 0)).is_err());
}

#[test]
fn get_digest_is_a_tpm2b_straight_after_the_header() {
    let mut r = header(0x8001, 0, 34);
    r.extend_from_slice(&[0, 32]);
    r.extend_from_slice(&[0xC3u8; 32]);
    assert_eq!(parse_get_digest(&r), Ok([0xC3u8; 32]));
}

#[test]
fn create_handle_is_read_before_the_parameter_area() {
    let mut r = header(0x8002, 0, 8);
    r.extend_from_slice(&[0x80, 0, 0, 0x01]);
    r.extend_from_slice(&[0, 0, 0, 0]);
    assert_eq!(parse_create(&r), Ok(0x8000_0001));
}

#[test]
fn hmac_digest_sits_behind_the_parameter_size() {
    let mut r = header(0x8002, 0, 38);
    r.extend_from_slice(&34u32.to_be_bytes());
    r.extend_from_slice(&[0, 32]);
    r.extend_from_slice(&[0x77u8; 32]);
    assert_eq!(parse_hmac(&r), Ok([0x77u8; 32]));
}

#[test]
fn hmac_without_the_parameter_size_would_misparse_and_is_refused() {
    /*
     * The NO_SESSIONS layout: digest at 10. Reading it as a SESSIONS response
     * finds a size of 0x7777 at 14 and must fail rather than return garbage.
     */
    let mut r = header(0x8001, 0, 34);
    r.extend_from_slice(&[0, 32]);
    r.extend_from_slice(&[0x77u8; 32]);
    assert!(parse_hmac(&r).is_err());
}
