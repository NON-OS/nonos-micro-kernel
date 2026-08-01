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

//! The issuance decision end to end: a receipt in, a grant out, once.

extern crate alloc;

use alloc::string::String;

use nonos_nox_license::{price_of, ToolId, TREASURY};
use nonos_nox_receipt::{ReceiptError, NOX_TOKEN, TRANSFER_TOPIC};

use crate::issue::{issue, IssueError};
use crate::spent::{SpentSet, SPENT_CAPACITY};

const BUYER: [u8; 20] = [
    0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
    0x00, 0x11, 0x22, 0x33,
];

fn hex_of(bytes: &[u8]) -> String {
    let mut s = String::from("0x");
    for b in bytes {
        s.push(char::from_digit((b >> 4) as u32, 16).unwrap());
        s.push(char::from_digit((b & 0xF) as u32, 16).unwrap());
    }
    s
}

fn addr_topic(addr: &[u8; 20]) -> String {
    let mut word = [0u8; 32];
    word[12..].copy_from_slice(addr);
    hex_of(&word)
}

fn amount_word(v: u128) -> String {
    let mut word = [0u8; 32];
    word[16..].copy_from_slice(&v.to_be_bytes());
    hex_of(&word)
}

/// A receipt paying `amount` NOX from BUYER to the treasury.
fn paying(amount: u128) -> String {
    let tt = hex_of(&TRANSFER_TOPIC);
    let from = addr_topic(&BUYER);
    let to = addr_topic(&TREASURY);
    let data = amount_word(amount);
    let nox = hex_of(&NOX_TOKEN);
    alloc::format!(
        "{{\"result\":{{\"status\":\"0x1\",\"logs\":[{{\"address\":\"{nox}\",\"topics\":[\"{tt}\",\"{from}\",\"{to}\"],\"data\":\"{data}\"}}]}}}}"
    )
}

fn price() -> u128 {
    price_of(ToolId::Recon.id()).unwrap()
}

#[test]
fn a_paid_receipt_issues_a_single_use_grant() {
    let mut spent = SpentSet::new();
    let tx = [0xAB; 32];
    let r = paying(price());
    let out =
        issue(&mut spent, ToolId::Recon.id(), tx, r.as_bytes(), 1_000, 3_600, [7; 8], [0; 32])
            .expect("valid payment issues");
    assert_eq!(out.entitlement.buyer, BUYER);
    assert_eq!(out.entitlement.uses, 1);
    assert_eq!(out.entitlement.tool_id, ToolId::Recon.id());
    assert_eq!(out.entitlement.tx_hash, tx);
    assert_eq!(out.entitlement.expiry, 1_000 + 3_600);
    assert!(spent.contains(&tx));
}

#[test]
fn paying_a_multiple_buys_a_bundle() {
    let mut spent = SpentSet::new();
    let r = paying(price() * 4);
    let out = issue(&mut spent, ToolId::Recon.id(), [1; 32], r.as_bytes(), 0, 0, [0; 8], [0; 32])
        .expect("valid");
    assert_eq!(out.entitlement.uses, 4);
    // ttl 0 means no expiry.
    assert_eq!(out.entitlement.expiry, 0);
}

#[test]
fn a_partial_overpayment_rounds_down_to_whole_uses() {
    let mut spent = SpentSet::new();
    // 2.5x the price: two whole uses, remainder is a tip.
    let r = paying(price() * 5 / 2);
    let out = issue(&mut spent, ToolId::Recon.id(), [2; 32], r.as_bytes(), 0, 0, [0; 8], [0; 32])
        .expect("valid");
    assert_eq!(out.entitlement.uses, 2);
}

#[test]
fn the_same_transaction_cannot_be_redeemed_twice() {
    let mut spent = SpentSet::new();
    let tx = [0xCD; 32];
    let r = paying(price());
    assert!(issue(&mut spent, ToolId::Recon.id(), tx, r.as_bytes(), 0, 0, [0; 8], [0; 32]).is_ok());
    // Same hash, same receipt: refused as replay.
    assert_eq!(
        issue(&mut spent, ToolId::Recon.id(), tx, r.as_bytes(), 0, 0, [1; 8], [0; 32]),
        Err(IssueError::Replay)
    );
}

#[test]
fn an_unknown_tool_is_refused_before_any_payment_check() {
    let mut spent = SpentSet::new();
    let r = paying(price());
    assert_eq!(
        issue(&mut spent, 999, [3; 32], r.as_bytes(), 0, 0, [0; 8], [0; 32]),
        Err(IssueError::UnknownTool)
    );
    // Nothing was recorded on a rejected request.
    assert!(spent.is_empty());
}

#[test]
fn an_underpaying_receipt_is_refused_and_records_nothing() {
    let mut spent = SpentSet::new();
    let tx = [4; 32];
    let r = paying(price() - 1);
    assert_eq!(
        issue(&mut spent, ToolId::Recon.id(), tx, r.as_bytes(), 0, 0, [0; 8], [0; 32]),
        Err(IssueError::Payment(ReceiptError::NoMatchingTransfer))
    );
    // A failed attempt must not burn the hash: the buyer can retry with a real
    // payment under the same tx once it confirms.
    assert!(!spent.contains(&tx));
}

#[test]
fn a_reverted_payment_is_refused() {
    let mut spent = SpentSet::new();
    let tt = hex_of(&TRANSFER_TOPIC);
    let from = addr_topic(&BUYER);
    let to = addr_topic(&TREASURY);
    let data = amount_word(price());
    let nox = hex_of(&NOX_TOKEN);
    let r = alloc::format!(
        "{{\"result\":{{\"status\":\"0x0\",\"logs\":[{{\"address\":\"{nox}\",\"topics\":[\"{tt}\",\"{from}\",\"{to}\"],\"data\":\"{data}\"}}]}}}}"
    );
    assert_eq!(
        issue(&mut spent, ToolId::Recon.id(), [5; 32], r.as_bytes(), 0, 0, [0; 8], [0; 32]),
        Err(IssueError::Payment(ReceiptError::Reverted))
    );
}

#[test]
fn the_spent_set_reports_when_it_wraps() {
    let mut spent = SpentSet::new();
    for i in 0..SPENT_CAPACITY {
        let mut tx = [0u8; 32];
        tx[0..8].copy_from_slice(&(i as u64).to_le_bytes());
        assert!(!spent.record(tx), "no eviction while filling");
    }
    assert_eq!(spent.len(), SPENT_CAPACITY);
    // One more evicts the oldest.
    assert!(spent.record([0xFF; 32]));
}

#[test]
fn device_binding_is_carried_into_the_grant() {
    let mut spent = SpentSet::new();
    let device = [0x5A; 32];
    let r = paying(price());
    let out = issue(&mut spent, ToolId::Recon.id(), [6; 32], r.as_bytes(), 0, 0, [0; 8], device)
        .expect("valid");
    assert_eq!(out.entitlement.device, device);
}
