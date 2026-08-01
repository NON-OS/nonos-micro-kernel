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

//! The format and the accept/reject gate, checked with real ed25519 signatures.
//!
//! The signer here is the same construction the broker uses in production, and
//! the verifier is the same math the kernel primitive runs, so a grant that
//! passes here is one the capsule would accept and one it would reject is one
//! the capsule would reject. Each rule is shown to reject on its own by
//! breaking exactly that field and leaving the rest of a good grant intact.

use crate::entitlement::{Entitlement, ParseError, ENTITLEMENT_LEN};
use crate::price::{price_of, tool_name, ToolId};
use crate::verify::{check, CheckError, Verify};

use ed25519_dalek::{Signer, SigningKey, VerifyingKey};

/// The broker: signs bodies with a real ed25519 key.
struct Broker {
    key: SigningKey,
}

impl Broker {
    fn new(seed: u8) -> Broker {
        Broker { key: SigningKey::from_bytes(&[seed; 32]) }
    }

    fn public(&self) -> BrokerKey {
        BrokerKey { key: self.key.verifying_key() }
    }

    fn issue(&self, ent: &Entitlement) -> [u8; ENTITLEMENT_LEN] {
        let sig = self.key.sign(&ent.body()).to_bytes();
        ent.encode(&sig)
    }
}

/// The verifier a tool holds: the broker's public key. `verify` is exactly
/// what the kernel ed25519 primitive computes.
struct BrokerKey {
    key: VerifyingKey,
}

impl Verify for BrokerKey {
    fn verify(&self, msg: &[u8], sig: &[u8; 64]) -> bool {
        let sig = ed25519_dalek::Signature::from_bytes(sig);
        self.key.verify_strict(msg, &sig).is_ok()
    }
}

fn sample() -> Entitlement {
    Entitlement {
        tool_id: ToolId::Recon.id(),
        buyer: [0x11; 20],
        device: [0u8; 32],
        uses: 1,
        issued_at: 1_000,
        expiry: 2_000,
        tx_hash: [0xAB; 32],
        nonce: [1, 2, 3, 4, 5, 6, 7, 8],
    }
}

#[test]
fn round_trips_through_bytes() {
    let ent = sample();
    let broker = Broker::new(7);
    let raw = broker.issue(&ent);
    let (back, _sig) = Entitlement::parse(&raw).expect("parse");
    assert_eq!(back, ent);
}

#[test]
fn a_valid_grant_is_accepted() {
    let ent = sample();
    let broker = Broker::new(7);
    let raw = broker.issue(&ent);
    let ok = check(&raw, ToolId::Recon.id(), 1_500, &[0u8; 32], &broker.public())
        .expect("valid grant must pass");
    assert_eq!(ok.buyer, ent.buyer);
    assert_eq!(ok.tx_hash, ent.tx_hash);
    assert_eq!(ok.uses, 1);
}

#[test]
fn a_forged_signature_is_rejected() {
    let ent = sample();
    let broker = Broker::new(7);
    let mut raw = broker.issue(&ent);
    // Flip a signature byte: the body still parses, the key does not vouch.
    let last = raw.len() - 1;
    raw[last] ^= 0x01;
    assert_eq!(
        check(&raw, ToolId::Recon.id(), 1_500, &[0u8; 32], &broker.public()),
        Err(CheckError::BadSignature)
    );
}

#[test]
fn a_grant_signed_by_the_wrong_key_is_rejected() {
    let ent = sample();
    let attacker = Broker::new(9);
    let raw = attacker.issue(&ent);
    let real = Broker::new(7);
    assert_eq!(
        check(&raw, ToolId::Recon.id(), 1_500, &[0u8; 32], &real.public()),
        Err(CheckError::BadSignature)
    );
}

#[test]
fn a_mutated_body_is_rejected() {
    let ent = sample();
    let broker = Broker::new(7);
    let mut raw = broker.issue(&ent);
    // Raise the granted uses without re-signing.
    raw[60] = 0xFF;
    assert_eq!(
        check(&raw, ToolId::Recon.id(), 1_500, &[0u8; 32], &broker.public()),
        Err(CheckError::BadSignature)
    );
}

#[test]
fn a_grant_for_another_tool_is_rejected() {
    let ent = sample();
    let broker = Broker::new(7);
    let raw = broker.issue(&ent);
    // Same signed grant, checked by a tool with a different id.
    assert_eq!(check(&raw, 999, 1_500, &[0u8; 32], &broker.public()), Err(CheckError::WrongTool));
}

#[test]
fn an_expired_grant_is_rejected() {
    let ent = sample();
    let broker = Broker::new(7);
    let raw = broker.issue(&ent);
    assert_eq!(
        check(&raw, ToolId::Recon.id(), 2_001, &[0u8; 32], &broker.public()),
        Err(CheckError::Expired)
    );
    // The boundary second is still inside the window.
    assert!(check(&raw, ToolId::Recon.id(), 2_000, &[0u8; 32], &broker.public()).is_ok());
}

#[test]
fn a_never_expiring_grant_survives_any_time() {
    let mut ent = sample();
    ent.expiry = 0;
    let broker = Broker::new(7);
    let raw = broker.issue(&ent);
    assert!(check(&raw, ToolId::Recon.id(), u64::MAX, &[0u8; 32], &broker.public()).is_ok());
}

#[test]
fn a_spent_grant_is_rejected() {
    let mut ent = sample();
    ent.uses = 0;
    let broker = Broker::new(7);
    let raw = broker.issue(&ent);
    assert_eq!(
        check(&raw, ToolId::Recon.id(), 1_500, &[0u8; 32], &broker.public()),
        Err(CheckError::NoUses)
    );
}

#[test]
fn a_device_bound_grant_needs_that_device() {
    let mut ent = sample();
    ent.device = [0x55; 32];
    let broker = Broker::new(7);
    let raw = broker.issue(&ent);
    // Right device passes.
    assert!(check(&raw, ToolId::Recon.id(), 1_500, &[0x55; 32], &broker.public()).is_ok());
    // A different device does not.
    assert_eq!(
        check(&raw, ToolId::Recon.id(), 1_500, &[0x66; 32], &broker.public()),
        Err(CheckError::WrongDevice)
    );
    // Nor does an install with no attestation to present.
    assert_eq!(
        check(&raw, ToolId::Recon.id(), 1_500, &[0u8; 32], &broker.public()),
        Err(CheckError::WrongDevice)
    );
}

#[test]
fn an_unbound_grant_runs_anywhere() {
    let ent = sample();
    let broker = Broker::new(7);
    let raw = broker.issue(&ent);
    assert!(check(&raw, ToolId::Recon.id(), 1_500, &[0xAA; 32], &broker.public()).is_ok());
}

#[test]
fn a_truncated_record_is_rejected() {
    let ent = sample();
    let broker = Broker::new(7);
    let raw = broker.issue(&ent);
    assert_eq!(
        check(&raw[..raw.len() - 1], ToolId::Recon.id(), 1_500, &[0u8; 32], &broker.public()),
        Err(CheckError::Malformed(ParseError::Length))
    );
}

#[test]
fn a_wrong_magic_is_rejected() {
    let ent = sample();
    let broker = Broker::new(7);
    let mut raw = broker.issue(&ent);
    raw[0] = b'X';
    assert_eq!(
        check(&raw, ToolId::Recon.id(), 1_500, &[0u8; 32], &broker.public()),
        Err(CheckError::Malformed(ParseError::Magic))
    );
}

#[test]
fn the_catalogue_is_priced_and_named() {
    assert_eq!(price_of(ToolId::Recon.id()), Some(250_000_000_000_000_000));
    assert_eq!(tool_name(ToolId::Recon.id()), Some("recon"));
    // An id we do not sell has neither a price nor a name, so it cannot be
    // charged for or advertised by accident.
    assert_eq!(price_of(0), None);
    assert_eq!(price_of(999), None);
    assert_eq!(tool_name(999), None);
}
