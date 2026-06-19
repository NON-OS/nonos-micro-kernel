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

extern crate alloc;

mod address;
mod rlp;
mod transaction;

pub use address::EthAddress;
pub use rlp::{rlp_encode_bytes, rlp_encode_list, rlp_encode_u128, rlp_encode_u64};
pub use transaction::{
    eth_sign_message, gwei_to_wei, parse_wei, wei_to_gwei, SignedTransaction, Transaction, Wallet,
};

pub const NOX_TOKEN_ADDRESS: [u8; 20] = hex_to_bytes_20("0a26c80Be4E060e688d7C23aDdB92cBb5D2C9eCA");

pub(crate) const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

const fn hex_to_bytes_20(hex: &str) -> [u8; 20] {
    let bytes = hex.as_bytes();
    let mut result = [0u8; 20];
    let mut i = 0;
    while i < 20 {
        result[i] = hex_byte(bytes[i * 2], bytes[i * 2 + 1]);
        i += 1;
    }
    result
}

const fn hex_byte(high: u8, low: u8) -> u8 {
    (hex_digit(high) << 4) | hex_digit(low)
}

pub(crate) const fn hex_digit(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => 0,
    }
}
