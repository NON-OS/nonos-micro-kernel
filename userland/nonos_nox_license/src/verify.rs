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

//! The one gate a tool calls before it runs.
//!
//! `check` parses a record, verifies its signature under the broker key
//! through a caller-supplied `Verify`, and enforces the field rules: right
//! tool, still valid at `now`, at least one use left, and if the grant is
//! device bound, bound to this device. It returns the fields only when every
//! check passes, so a caller cannot act on an entitlement it did not fully
//! validate.
//!
//! Signature checking is injected on purpose. A capsule passes the kernel
//! ed25519 primitive; a host test passes a real signer. The bytes and the
//! rules are identical either way, which is what makes the host proof mean
//! something for the capsule.

use crate::entitlement::{Entitlement, ParseError};

/// Verifies a detached ed25519 signature over `msg` under a fixed public key.
/// Returns true only for a good signature. An implementation must not panic
/// and must fail closed on any malformed input.
pub trait Verify {
    fn verify(&self, msg: &[u8], sig: &[u8; 64]) -> bool;
}

/// Why an entitlement was refused. Every variant is a refusal; there is no
/// "valid" variant, because validity is the `Ok` of `check`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CheckError {
    /// Not a well-formed record.
    Malformed(ParseError),
    /// Signature did not verify under the broker key.
    BadSignature,
    /// Grant is for a different tool than the one checking it.
    WrongTool,
    /// `now` is past `expiry`.
    Expired,
    /// Grant carries no remaining uses.
    NoUses,
    /// Grant is device bound and this is not that device.
    WrongDevice,
}

/// The fields of an accepted entitlement. Only `check` constructs one, so
/// holding a `Checked` is proof the record passed every rule.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Checked {
    pub buyer: [u8; 20],
    pub uses: u32,
    pub expiry: u64,
    pub tx_hash: [u8; 32],
    pub nonce: [u8; 8],
}

/// Accept `raw` for `expected_tool` at time `now`, or say why not.
///
/// `device` is this install's attestation. When a grant's device field is all
/// zero it is not bound and any device is accepted; otherwise it must match.
/// Pass an all-zero `device` on an install with no attestation to hand: an
/// unbound grant still verifies, a bound one does not, which is the safe way
/// round.
pub fn check(
    raw: &[u8],
    expected_tool: u32,
    now: u64,
    device: &[u8; 32],
    broker: &dyn Verify,
) -> Result<Checked, CheckError> {
    let (ent, sig) = Entitlement::parse(raw).map_err(CheckError::Malformed)?;

    // Signature first: nothing else in the record can be trusted until the
    // broker key has vouched for the exact bytes.
    if !broker.verify(&ent.body(), &sig) {
        return Err(CheckError::BadSignature);
    }
    if ent.tool_id != expected_tool {
        return Err(CheckError::WrongTool);
    }
    if ent.expiry != 0 && now > ent.expiry {
        return Err(CheckError::Expired);
    }
    if ent.uses == 0 {
        return Err(CheckError::NoUses);
    }
    if !device_ok(&ent.device, device) {
        return Err(CheckError::WrongDevice);
    }

    Ok(Checked {
        buyer: ent.buyer,
        uses: ent.uses,
        expiry: ent.expiry,
        tx_hash: ent.tx_hash,
        nonce: ent.nonce,
    })
}

/// An all-zero device field means unbound and matches anything; otherwise the
/// grant's device must equal this install's.
fn device_ok(bound: &[u8; 32], device: &[u8; 32]) -> bool {
    if bound.iter().all(|b| *b == 0) {
        return true;
    }
    bound == device
}
