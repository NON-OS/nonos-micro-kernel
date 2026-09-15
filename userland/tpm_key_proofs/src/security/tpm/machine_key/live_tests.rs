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

//! The sequence against a real TPM 2.0 implementation.
//!
//! What the byte tests cannot show: that the TPM accepts each command, that
//! the policy session authorises the HMAC with an empty auth, that the key is
//! stable across derivations and across sessions, and that extending a bound
//! PCR changes the key and makes the old policy fail. Skipped, loudly, when
//! `swtpm` is not installed.

use super::consts::{DIGEST_LEN, NONCE_LEN};
use super::create::{build_create, parse_create};
use super::error::KeyError;
use super::flush::build_flush;
use super::hmac::{build_hmac, parse_hmac};
use super::pcrs::BOUND_PCRS;
use super::policy::{build_get_digest, build_policy_pcr, parse_get_digest, parse_policy_pcr};
use super::session::{build_start, parse_start};
use super::swtpm::Swtpm;

const TPM_CC_PCR_EXTEND: u32 = 0x0000_0182;

fn tpm() -> Option<Swtpm> {
    let t = Swtpm::start();
    if t.is_none() {
        eprintln!("swtpm not installed: live derivation not exercised");
    }
    t
}

/// The kernel's `derive`, with the transport swapped. Returns the policy
/// digest too, so a test can pin a template and watch it fail later.
fn derive(t: &mut Swtpm, label: &[u8]) -> Result<([u8; DIGEST_LEN], [u8; DIGEST_LEN]), KeyError> {
    let session = parse_start(&t.run(&build_start(&[0x42; NONCE_LEN])))?;
    let r = (|| {
        parse_policy_pcr(&t.run(&build_policy_pcr(session, &BOUND_PCRS)))?;
        let policy = parse_get_digest(&t.run(&build_get_digest(session)))?;
        let key = parse_create(&t.run(&build_create(&policy)))?;
        let r = parse_hmac(&t.run(&build_hmac(key, session, label)));
        t.run(&build_flush(key));
        r.map(|k| (k, policy))
    })();
    t.run(&build_flush(session));
    r
}

fn extend(t: &mut Swtpm, pcr: u32, digest: &[u8; 32]) {
    let mut body = pcr.to_be_bytes().to_vec();
    body.extend_from_slice(&9u32.to_be_bytes());
    body.extend_from_slice(&[0x40, 0, 0, 0x09, 0, 0, 0, 0, 0]);
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&[0, 0x0B]);
    body.extend_from_slice(digest);
    let cmd = super::wire::frame(0x8002, TPM_CC_PCR_EXTEND, &body);
    let resp = t.run(&cmd);
    assert_eq!(&resp[6..10], &[0, 0, 0, 0], "pcr extend accepted");
}

#[test]
fn same_label_same_key_across_sessions() {
    let Some(mut t) = tpm() else { return };
    let (a, _) = derive(&mut t, b"nonos.volume.v1").expect("first derivation");
    let (b, _) = derive(&mut t, b"nonos.volume.v1").expect("second derivation");
    assert_eq!(a, b);
    assert_ne!(a, [0u8; 32]);
}

#[test]
fn different_labels_different_keys() {
    let Some(mut t) = tpm() else { return };
    let (v, _) = derive(&mut t, b"nonos.volume.v1").unwrap();
    let (w, _) = derive(&mut t, b"nonos.wallet.v1").unwrap();
    assert_ne!(v, w);
}

#[test]
fn slots_are_returned_so_the_sequence_repeats_indefinitely() {
    let Some(mut t) = tpm() else { return };
    for _ in 0..12 {
        derive(&mut t, b"again").expect("a derivation after many");
    }
}

#[test]
fn extending_a_bound_pcr_changes_the_key() {
    let Some(mut t) = tpm() else { return };
    let (before, _) = derive(&mut t, b"nonos.volume.v1").unwrap();
    extend(&mut t, 9, &[0xEE; 32]);
    let (after, _) = derive(&mut t, b"nonos.volume.v1").unwrap();
    assert_ne!(before, after);
}

#[test]
fn extending_an_unbound_pcr_does_not() {
    let Some(mut t) = tpm() else { return };
    let (before, _) = derive(&mut t, b"nonos.volume.v1").unwrap();
    extend(&mut t, 16, &[0xEE; 32]);
    let (after, _) = derive(&mut t, b"nonos.volume.v1").unwrap();
    assert_eq!(before, after);
}

#[test]
fn the_old_template_cannot_be_used_from_the_wrong_state() {
    let Some(mut t) = tpm() else { return };
    let (_, good_policy) = derive(&mut t, b"x").unwrap();
    extend(&mut t, 9, &[0xEE; 32]);
    /*
     * An attacker in the new state re-creates the object from the old policy
     * digest, which they can know. The derivation gives back the old key
     * material inside the TPM, but a session satisfied by the real PCRs no
     * longer matches that policy, and the HMAC is refused.
     */
    let session = parse_start(&t.run(&build_start(&[1; NONCE_LEN]))).unwrap();
    parse_policy_pcr(&t.run(&build_policy_pcr(session, &BOUND_PCRS))).unwrap();
    let key = parse_create(&t.run(&build_create(&good_policy))).unwrap();
    let r = parse_hmac(&t.run(&build_hmac(key, session, b"x")));
    t.run(&build_flush(key));
    t.run(&build_flush(session));
    match r {
        Err(KeyError::Refused(rc)) => {
            assert_eq!(rc & 0xFFF, 0x99D, "TPM_RC_POLICY_FAIL, got {rc:#x}")
        }
        other => panic!("expected policy failure, got {other:?}"),
    }
}
