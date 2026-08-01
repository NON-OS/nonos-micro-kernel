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

//! The issuance decision.

use nonos_nox_license::{price_of, Entitlement, TREASURY};
use nonos_nox_receipt::{verify_payment, ReceiptError, NOX_TOKEN};

use crate::spent::SpentSet;

/// A granted entitlement, not yet signed, with the terms the broker settled on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Issued {
    /// The grant to sign and return to the buyer.
    pub entitlement: Entitlement,
    /// Whether recording the funding hash evicted an older one, i.e. the
    /// spent-set is at capacity and should be widened.
    pub spent_set_full: bool,
}

/// Why the broker declined to issue.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IssueError {
    /// The tool id is not one we sell, so it has no price.
    UnknownTool,
    /// The receipt does not prove a sufficient NOX payment to the treasury.
    Payment(ReceiptError),
    /// This transaction was already redeemed for a grant.
    Replay,
}

/// Decide whether the receipt earns an entitlement for `tool_id`.
///
/// On success the funding hash is recorded in `spent` and the returned grant is
/// ready to sign. `now` and `ttl_secs` set the validity window; a zero
/// `ttl_secs` makes the grant never expire. `nonce` and `device` are the
/// broker's to choose: `device` binds the grant to one install (all zero leaves
/// it unbound). The number of uses is the payment divided by the unit price, so
/// paying a multiple of the price buys that many runs at once.
#[allow(clippy::too_many_arguments)]
pub fn issue(
    spent: &mut SpentSet,
    tool_id: u32,
    tx_hash: [u8; 32],
    receipt_json: &[u8],
    now: u64,
    ttl_secs: u64,
    nonce: [u8; 8],
    device: [u8; 32],
) -> Result<Issued, IssueError> {
    let price = price_of(tool_id).ok_or(IssueError::UnknownTool)?;

    let payment =
        verify_payment(receipt_json, &TREASURY, price, &NOX_TOKEN).map_err(IssueError::Payment)?;

    // Check replay before recording so a rejected receipt never marks a hash,
    // and record only once everything else has passed so a failure below cannot
    // burn the payment.
    if spent.contains(&tx_hash) {
        return Err(IssueError::Replay);
    }

    // Price is non-zero (it came from the catalogue), so the division is safe
    // and at least one, since the payment cleared the price.
    let uses = (payment.amount / price).min(u32::MAX as u128) as u32;

    let expiry = if ttl_secs == 0 { 0 } else { now.saturating_add(ttl_secs) };

    let entitlement = Entitlement {
        tool_id,
        buyer: payment.from,
        device,
        uses,
        issued_at: now,
        expiry,
        tx_hash,
        nonce,
    };

    let spent_set_full = spent.record(tx_hash);
    Ok(Issued { entitlement, spent_set_full })
}
