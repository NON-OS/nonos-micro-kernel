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

//! The entitlement record and its canonical bytes.
//!
//! Layout is fixed and little-endian. The signature covers exactly the first
//! `BODY_LEN` bytes, so serialize and parse agree on the signed region without
//! either side re-deriving it. Every field is a fixed width, so there is one
//! encoding of a given entitlement and nothing to disagree about.

/// Four magic bytes, last one a format version. Bumping the version changes
/// these bytes, so an old verifier rejects a new record instead of
/// misreading it.
pub const MAGIC: [u8; 4] = *b"NXL1";

/// Signed region: everything except the trailing signature.
pub const BODY_LEN: usize = 4    // magic
    + 4    // tool_id
    + 20   // buyer
    + 32   // device
    + 4    // uses
    + 8    // issued_at
    + 8    // expiry
    + 32   // tx_hash
    + 8; // nonce

/// Detached ed25519 signature length.
pub const SIG_LEN: usize = 64;

/// Full on-wire entitlement: body followed by its signature.
pub const ENTITLEMENT_LEN: usize = BODY_LEN + SIG_LEN;

/// A grant the broker issued against a confirmed NOX payment.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Entitlement {
    /// Which tool this grant unlocks.
    pub tool_id: u32,
    /// The address that paid, low 20 bytes of the NOX/eth account.
    pub buyer: [u8; 20],
    /// Attestation the grant is bound to (boot or policy root). All zero means
    /// the grant is not device bound and any NONOS install may present it.
    pub device: [u8; 32],
    /// Runs this payment bought. One for a single pay-per-use unlock; more for
    /// a bundle. A verifier treats zero as no entitlement.
    pub uses: u32,
    /// Unix seconds the broker signed at.
    pub issued_at: u64,
    /// Unix seconds after which the grant is dead. Zero never expires.
    pub expiry: u64,
    /// The NOX transaction that funded this grant. The broker keys its
    /// spent-set on this, so one payment yields exactly one entitlement.
    pub tx_hash: [u8; 32],
    /// Broker-chosen uniqueness so two grants for the same payment terms still
    /// differ byte for byte.
    pub nonce: [u8; 8],
}

/// Why a byte slice is not a well-formed entitlement.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParseError {
    /// Slice was not exactly `ENTITLEMENT_LEN` bytes.
    Length,
    /// Leading bytes were not `MAGIC`.
    Magic,
}

impl Entitlement {
    /// Serialize the signed body. The caller signs these bytes and appends the
    /// signature to form the full record.
    pub fn body(&self) -> [u8; BODY_LEN] {
        let mut b = [0u8; BODY_LEN];
        b[0..4].copy_from_slice(&MAGIC);
        b[4..8].copy_from_slice(&self.tool_id.to_le_bytes());
        b[8..28].copy_from_slice(&self.buyer);
        b[28..60].copy_from_slice(&self.device);
        b[60..64].copy_from_slice(&self.uses.to_le_bytes());
        b[64..72].copy_from_slice(&self.issued_at.to_le_bytes());
        b[72..80].copy_from_slice(&self.expiry.to_le_bytes());
        b[80..112].copy_from_slice(&self.tx_hash);
        b[112..120].copy_from_slice(&self.nonce);
        b
    }

    /// Assemble the full record from a body and its detached signature.
    pub fn encode(&self, sig: &[u8; SIG_LEN]) -> [u8; ENTITLEMENT_LEN] {
        let mut out = [0u8; ENTITLEMENT_LEN];
        out[..BODY_LEN].copy_from_slice(&self.body());
        out[BODY_LEN..].copy_from_slice(sig);
        out
    }

    /// Recover the fields from a full record. The signature is not checked
    /// here; parsing is only the shape, and `verify::check` is the gate.
    pub fn parse(raw: &[u8]) -> Result<(Entitlement, [u8; SIG_LEN]), ParseError> {
        if raw.len() != ENTITLEMENT_LEN {
            return Err(ParseError::Length);
        }
        if raw[0..4] != MAGIC {
            return Err(ParseError::Magic);
        }
        let tool_id = u32::from_le_bytes([raw[4], raw[5], raw[6], raw[7]]);
        let mut buyer = [0u8; 20];
        buyer.copy_from_slice(&raw[8..28]);
        let mut device = [0u8; 32];
        device.copy_from_slice(&raw[28..60]);
        let uses = u32::from_le_bytes([raw[60], raw[61], raw[62], raw[63]]);
        let issued_at = u64::from_le_bytes([
            raw[64], raw[65], raw[66], raw[67], raw[68], raw[69], raw[70], raw[71],
        ]);
        let expiry = u64::from_le_bytes([
            raw[72], raw[73], raw[74], raw[75], raw[76], raw[77], raw[78], raw[79],
        ]);
        let mut tx_hash = [0u8; 32];
        tx_hash.copy_from_slice(&raw[80..112]);
        let mut nonce = [0u8; 8];
        nonce.copy_from_slice(&raw[112..120]);
        let mut sig = [0u8; SIG_LEN];
        sig.copy_from_slice(&raw[BODY_LEN..]);
        Ok((Entitlement { tool_id, buyer, device, uses, issued_at, expiry, tx_hash, nonce }, sig))
    }
}
