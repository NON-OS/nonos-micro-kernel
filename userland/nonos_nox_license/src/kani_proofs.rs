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

//! Kani harnesses: the format round-trips and the gate is sound and complete
//! for every input, not just the sampled ones.

use crate::entitlement::Entitlement;
use crate::verify::{check, Verify};

/// A verifier whose answer is fixed by construction, modelling "the signature
/// is valid" or "is not" without the ed25519 math, so the harness reasons
/// about the gate's logic over both cases.
struct Fixed(bool);

impl Verify for Fixed {
    fn verify(&self, _msg: &[u8], _sig: &[u8; 64]) -> bool {
        self.0
    }
}

fn any_entitlement() -> Entitlement {
    Entitlement {
        tool_id: kani::any(),
        buyer: kani::any(),
        device: kani::any(),
        uses: kani::any(),
        issued_at: kani::any(),
        expiry: kani::any(),
        tx_hash: kani::any(),
        nonce: kani::any(),
    }
}

// Encoding then parsing recovers the same fields and signature, for every
// entitlement and signature.
#[kani::proof]
fn parse_inverts_encode() {
    let ent = any_entitlement();
    let sig: [u8; 64] = kani::any();
    let raw = ent.encode(&sig);
    match Entitlement::parse(&raw) {
        Ok((back, sig_back)) => {
            assert!(back == ent);
            assert!(sig_back == sig);
        }
        Err(_) => assert!(false, "a well-formed record must parse"),
    }
}

// Acceptance implies every rule held: a valid signature, the right tool, a live
// grant with a use left, and the right device. Nothing slips past the gate.
#[kani::proof]
fn acceptance_implies_every_rule() {
    let ent = any_entitlement();
    let sig: [u8; 64] = kani::any();
    let raw = ent.encode(&sig);

    let expected_tool: u32 = kani::any();
    let now: u64 = kani::any();
    let device: [u8; 32] = kani::any();
    let sig_ok: bool = kani::any();

    if check(&raw, expected_tool, now, &device, &Fixed(sig_ok)).is_ok() {
        assert!(sig_ok);
        assert!(ent.tool_id == expected_tool);
        assert!(ent.expiry == 0 || now <= ent.expiry);
        assert!(ent.uses > 0);
        let unbound = ent.device == [0u8; 32];
        assert!(unbound || ent.device == device);
    }
}

// The converse: when every rule holds the gate accepts, so a paying buyer is
// never wrongly refused.
#[kani::proof]
fn every_rule_holding_implies_acceptance() {
    let ent = any_entitlement();
    let sig: [u8; 64] = kani::any();
    let raw = ent.encode(&sig);

    let now: u64 = kani::any();
    let device: [u8; 32] = kani::any();

    let unbound = ent.device == [0u8; 32];
    kani::assume(ent.uses > 0);
    kani::assume(ent.expiry == 0 || now <= ent.expiry);
    kani::assume(unbound || ent.device == device);

    assert!(check(&raw, ent.tool_id, now, &device, &Fixed(true)).is_ok());
}
