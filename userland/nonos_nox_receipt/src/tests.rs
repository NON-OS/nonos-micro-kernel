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

//! Payment verification against real-shaped receipt JSON.
//!
//! Topics and amounts are built from typed values so the fixtures cannot drift
//! from what they claim to encode. One good receipt is accepted, and every way
//! a receipt can fail to prove the payment is shown to be refused: reverted, no
//! status, wrong token, wrong recipient, too little, a non-transfer event, and
//! a decoy transfer beside the real one.

extern crate alloc;

use alloc::string::String;

use crate::verify::{verify_payment, Payment, ReceiptError, NOX_TOKEN, TRANSFER_TOPIC};

const TREASURY: [u8; 20] = [
    0x4e, 0x4f, 0x4e, 0x4f, 0x53, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x01,
];
const PRICE: u128 = 250_000_000_000_000_000;
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

/// A 20-byte address as a left-padded 32-byte indexed topic.
fn addr_topic(addr: &[u8; 20]) -> String {
    let mut word = [0u8; 32];
    word[12..].copy_from_slice(addr);
    hex_of(&word)
}

/// A u128 amount as a 32-byte EVM word.
fn amount_word(v: u128) -> String {
    let mut word = [0u8; 32];
    word[16..].copy_from_slice(&v.to_be_bytes());
    hex_of(&word)
}

fn transfer_topic() -> String {
    hex_of(&TRANSFER_TOPIC)
}

/// A receipt carrying one log with the given parts.
fn receipt(status: &str, addr: &str, topics: &[String], data: &str) -> String {
    let mut t = String::new();
    for (i, top) in topics.iter().enumerate() {
        if i > 0 {
            t.push(',');
        }
        t.push('"');
        t.push_str(top);
        t.push('"');
    }
    alloc::format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{{\"status\":\"{status}\",\"logs\":[{{\"address\":\"{addr}\",\"topics\":[{t}],\"data\":\"{data}\"}}]}}}}"
    )
}

fn nox_addr() -> String {
    hex_of(&NOX_TOKEN)
}

#[test]
fn a_valid_payment_is_accepted() {
    let topics = [transfer_topic(), addr_topic(&BUYER), addr_topic(&TREASURY)];
    let r = receipt("0x1", &nox_addr(), &topics, &amount_word(PRICE));
    let p = verify_payment(r.as_bytes(), &TREASURY, PRICE, &NOX_TOKEN).expect("valid");
    assert_eq!(p, Payment { from: BUYER, amount: PRICE });
}

#[test]
fn overpayment_is_accepted_and_reported() {
    let topics = [transfer_topic(), addr_topic(&BUYER), addr_topic(&TREASURY)];
    let r = receipt("0x1", &nox_addr(), &topics, &amount_word(PRICE * 2));
    let p = verify_payment(r.as_bytes(), &TREASURY, PRICE, &NOX_TOKEN).expect("valid");
    assert_eq!(p.amount, PRICE * 2);
}

#[test]
fn a_reverted_transaction_is_rejected() {
    let topics = [transfer_topic(), addr_topic(&BUYER), addr_topic(&TREASURY)];
    let r = receipt("0x0", &nox_addr(), &topics, &amount_word(PRICE));
    assert_eq!(
        verify_payment(r.as_bytes(), &TREASURY, PRICE, &NOX_TOKEN),
        Err(ReceiptError::Reverted)
    );
}

#[test]
fn a_receipt_without_status_is_rejected() {
    let r = "{\"result\":{\"logs\":[]}}";
    assert_eq!(
        verify_payment(r.as_bytes(), &TREASURY, PRICE, &NOX_TOKEN),
        Err(ReceiptError::NoStatus)
    );
}

#[test]
fn a_transfer_of_another_token_is_rejected() {
    let other = [0x11u8; 20];
    let topics = [transfer_topic(), addr_topic(&BUYER), addr_topic(&TREASURY)];
    let r = receipt("0x1", &hex_of(&other), &topics, &amount_word(PRICE));
    assert_eq!(
        verify_payment(r.as_bytes(), &TREASURY, PRICE, &NOX_TOKEN),
        Err(ReceiptError::NoMatchingTransfer)
    );
}

#[test]
fn a_transfer_to_someone_else_is_rejected() {
    let elsewhere = [0xDEu8; 20];
    let topics = [transfer_topic(), addr_topic(&BUYER), addr_topic(&elsewhere)];
    let r = receipt("0x1", &nox_addr(), &topics, &amount_word(PRICE));
    assert_eq!(
        verify_payment(r.as_bytes(), &TREASURY, PRICE, &NOX_TOKEN),
        Err(ReceiptError::NoMatchingTransfer)
    );
}

#[test]
fn an_underpayment_is_rejected() {
    let topics = [transfer_topic(), addr_topic(&BUYER), addr_topic(&TREASURY)];
    let r = receipt("0x1", &nox_addr(), &topics, &amount_word(PRICE - 1));
    assert_eq!(
        verify_payment(r.as_bytes(), &TREASURY, PRICE, &NOX_TOKEN),
        Err(ReceiptError::NoMatchingTransfer)
    );
    // The exact price is enough; underpayment is strictly below.
    let ok = receipt("0x1", &nox_addr(), &topics, &amount_word(PRICE));
    assert!(verify_payment(ok.as_bytes(), &TREASURY, PRICE, &NOX_TOKEN).is_ok());
}

#[test]
fn a_non_transfer_event_is_ignored() {
    // Approval(address,address,uint256) topic0, not Transfer.
    let approval = [
        0x8c, 0x5b, 0xe1, 0xe5, 0xeb, 0xec, 0x7d, 0x5b, 0xd1, 0x4f, 0x71, 0x42, 0x7d, 0x1e, 0x84,
        0xf3, 0xdd, 0x03, 0x14, 0xc0, 0xf7, 0xb2, 0x29, 0x1e, 0x5b, 0x20, 0x0a, 0xc8, 0xc7, 0xc3,
        0xb9, 0x25,
    ];
    let topics = [hex_of(&approval), addr_topic(&BUYER), addr_topic(&TREASURY)];
    let r = receipt("0x1", &nox_addr(), &topics, &amount_word(PRICE));
    assert_eq!(
        verify_payment(r.as_bytes(), &TREASURY, PRICE, &NOX_TOKEN),
        Err(ReceiptError::NoMatchingTransfer)
    );
}

#[test]
fn a_decoy_transfer_does_not_hide_the_real_one() {
    let decoy = [0xDEu8; 20];
    let tt = transfer_topic();
    let buyer = addr_topic(&BUYER);
    let decoy_to = addr_topic(&decoy);
    let treasury_to = addr_topic(&TREASURY);
    let amt = amount_word(PRICE);
    let nox = nox_addr();
    let r = alloc::format!(
        "{{\"result\":{{\"status\":\"0x1\",\"logs\":[\
         {{\"address\":\"{nox}\",\"topics\":[\"{tt}\",\"{buyer}\",\"{decoy_to}\"],\"data\":\"{amt}\"}},\
         {{\"address\":\"{nox}\",\"topics\":[\"{tt}\",\"{buyer}\",\"{treasury_to}\"],\"data\":\"{amt}\"}}\
         ]}}}}"
    );
    let p = verify_payment(r.as_bytes(), &TREASURY, PRICE, &NOX_TOKEN).expect("second log pays");
    assert_eq!(p.from, BUYER);
}

#[test]
fn a_truncated_receipt_fails_closed() {
    let topics = [transfer_topic(), addr_topic(&BUYER), addr_topic(&TREASURY)];
    let r = receipt("0x1", &nox_addr(), &topics, &amount_word(PRICE));
    let cut = &r.as_bytes()[..r.len() / 2];
    assert!(verify_payment(cut, &TREASURY, PRICE, &NOX_TOKEN).is_err());
}
