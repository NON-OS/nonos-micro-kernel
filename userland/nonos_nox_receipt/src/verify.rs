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

//! The payment decision: does this receipt fund the tool?

use crate::hex::{decode_fixed, decode_u128};
use crate::logs::{for_each_log, status_ok, RawLog};

/// `keccak256("Transfer(address,address,uint256)")`, topic zero of every ERC20
/// transfer log. A log whose first topic is not this is not a transfer and is
/// ignored, so a receipt cannot pass by carrying some other event.
pub const TRANSFER_TOPIC: [u8; 32] = [
    0xdd, 0xf2, 0x52, 0xad, 0x1b, 0xe2, 0xc8, 0x9b, 0x69, 0xc2, 0xb0, 0x68, 0xfc, 0x37, 0x8d, 0xaa,
    0x95, 0x2b, 0xa7, 0xf1, 0x63, 0xc4, 0xa1, 0x16, 0x28, 0xf5, 0x5a, 0x4d, 0xf5, 0x23, 0xb3, 0xef,
];

/// The NOX token contract. A transfer must be emitted by this address, so
/// paying in some other ERC20 does not unlock a tool. Matches the wallet's
/// `nox::constants::NOX_TOKEN`.
pub const NOX_TOKEN: [u8; 20] = [
    0x0a, 0x26, 0xc8, 0x0b, 0xe4, 0xe0, 0x60, 0xe6, 0x88, 0xd7, 0xc2, 0x3a, 0xdd, 0xb9, 0x2c, 0xbb,
    0x5d, 0x2c, 0x9e, 0xca,
];

/// A confirmed NOX transfer to the treasury.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Payment {
    /// The paying account, from the transfer's first indexed argument.
    pub from: [u8; 20],
    /// Amount transferred, in NOX base units.
    pub amount: u128,
}

/// Why a receipt does not prove the payment.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReceiptError {
    /// No `status` field, so the transaction's success is unknown.
    NoStatus,
    /// Transaction reverted (`status` `0x0`).
    Reverted,
    /// No NOX transfer to the treasury for at least the price was found.
    NoMatchingTransfer,
}

/// Accept the receipt only if it shows a successful transaction with a NOX
/// `Transfer` to `treasury` of at least `min_amount`, and report who paid and
/// how much. `token` is the contract the transfer must come from; pass
/// [`NOX_TOKEN`].
pub fn verify_payment(
    receipt_json: &[u8],
    treasury: &[u8; 20],
    min_amount: u128,
    token: &[u8; 20],
) -> Result<Payment, ReceiptError> {
    match status_ok(receipt_json) {
        Some(true) => {}
        Some(false) => return Err(ReceiptError::Reverted),
        None => return Err(ReceiptError::NoStatus),
    }

    let mut found: Option<Payment> = None;
    for_each_log(receipt_json, |log| match transfer_to(log, treasury, min_amount, token) {
        Some(p) => {
            found = Some(p);
            true
        }
        None => false,
    });

    found.ok_or(ReceiptError::NoMatchingTransfer)
}

/// If this log is a NOX transfer to `treasury` for at least `min_amount`,
/// return the payment; otherwise `None`.
fn transfer_to(
    log: &RawLog<'_>,
    treasury: &[u8; 20],
    min_amount: u128,
    token: &[u8; 20],
) -> Option<Payment> {
    // Emitted by the NOX token.
    let mut addr = [0u8; 20];
    if !decode_fixed(log.address, &mut addr) || &addr != token {
        return None;
    }
    // Transfer(from, to, value): topic0 signature, topic1 from, topic2 to.
    if log.topic_count < 3 {
        return None;
    }
    let mut topic0 = [0u8; 32];
    if !decode_fixed(log.topics[0], &mut topic0) || topic0 != TRANSFER_TOPIC {
        return None;
    }
    // Indexed address topics are a full 32-byte word, address in the low 20.
    let mut from_word = [0u8; 32];
    if !decode_fixed(log.topics[1], &mut from_word) {
        return None;
    }
    let mut to_word = [0u8; 32];
    if !decode_fixed(log.topics[2], &mut to_word) {
        return None;
    }
    if &to_word[12..32] != treasury {
        return None;
    }
    let amount = decode_u128(log.data)?;
    if amount < min_amount {
        return None;
    }
    let mut from = [0u8; 20];
    from.copy_from_slice(&from_word[12..32]);
    Some(Payment { from, amount })
}
