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

//! Known-answer checks on the real wallet helpers, verified against Foundry
//! `cast` and live Ethereum mainnet vectors.

use crate::hex_digit::hex_digit;
use crate::nox::apr_bps::apr_bps;
use crate::nox::calldata_addr::calldata_addr;
use crate::nox::constants::SEL_BALANCE_OF;
use crate::nox::format_apr::format_apr;
use crate::nox::format_nox::format_nox;
use crate::nox::q32_to_u128::q32_to_u128;

/// `cast calldata "balanceOf(address)" 0xd8dA...6045` byte for byte.
#[test]
fn calldata_matches_cast() {
    let vitalik: [u8; 20] = [
        0xd8, 0xda, 0x6b, 0xf2, 0x69, 0x64, 0xaf, 0x9d, 0x7e, 0xed, 0x9e, 0x03, 0xe5, 0x34, 0x15,
        0xd3, 0x7a, 0xa9, 0x60, 0x45,
    ];
    let cd = calldata_addr(&SEL_BALANCE_OF, &vitalik);
    let hex: String = cd.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(hex, "70a08231000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045");
}

/// Live mainnet: emission 8.878e17, total staked 2.170e26 -> 1290 bps (12.90%).
#[test]
fn apr_matches_live_mainnet() {
    let bps = apr_bps(887_874_175_545_408_422, 217_029_595_357_176_730_748_693_395);
    assert_eq!(bps, Some(1290));
}

#[test]
fn apr_empty_pool_is_none() {
    assert_eq!(apr_bps(1_000_000, 0), None);
}

#[test]
fn quantity_rejects_high_bytes() {
    let mut w = [0u8; 32];
    w[0] = 1;
    assert_eq!(q32_to_u128(&w), None);
}

#[test]
fn quantity_decodes_low_bytes() {
    let mut w = [0u8; 32];
    w[31] = 0xff;
    w[30] = 0x01;
    assert_eq!(q32_to_u128(&w), Some(0x01ff));
}

#[test]
fn format_nox_two_decimals() {
    let mut buf = [0u8; 48];
    let n = format_nox(1_500_000_000_000_000_000, &mut buf);
    assert_eq!(&buf[..n], b"1.50");
}

#[test]
fn format_apr_percent() {
    let mut buf = [0u8; 16];
    let n = format_apr(1290, &mut buf);
    assert_eq!(&buf[..n], b"12.90%");
}

#[test]
fn hex_digit_alphabets() {
    assert_eq!(hex_digit(b'a' as u32), Some(10));
    assert_eq!(hex_digit(b'F' as u32), Some(15));
    assert_eq!(hex_digit(b'9' as u32), Some(9));
    assert_eq!(hex_digit(b'z' as u32), None);
}
