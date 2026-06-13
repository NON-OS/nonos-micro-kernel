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

pub fn self_check() -> bool {
    let addr = [0x11u8; 20];
    let hash = [0x22u8; 32];
    let raw = [0x02u8, 0xf8, 0x01];
    let balance = b"{\"jsonrpc\":\"2.0\",\"result\":\"0xde0b6b3a7640000\",\"id\":1}";
    let tx = b"{\"jsonrpc\":\"2.0\",\"result\":\"0x2222222222222222222222222222222222222222222222222222222222222222\",\"id\":4}";
    let q = match super::parse_quantity32::parse_quantity32(balance) {
        Some(q) => q,
        None => return false,
    };
    super::request_chain_id::request_chain_id(1).starts_with(b"{\"jsonrpc\"")
        && super::request_balance::request_balance(&addr, 2).windows(14).any(|w| w == b"eth_getBalance")
        && super::request_nonce::request_nonce(&addr, 3).windows(23).any(|w| w == b"eth_getTransactionCount")
        && super::request_fee::request_fee(4).windows(12).any(|w| w == b"eth_gasPrice")
        && super::request_broadcast::request_broadcast(&raw, 5).windows(22).any(|w| w == b"eth_sendRawTransaction")
        && super::request_receipt::request_receipt(&hash, 6).windows(25).any(|w| w == b"eth_getTransactionReceipt")
        && super::parse_u64::parse_u64(b"{\"result\":\"0x1\"}") == Some(1)
        && super::parse_hash32::parse_hash32(tx) == Some(hash)
        && super::parse_receipt_ok::parse_receipt_ok(b"{\"result\":{\"status\":\"0x1\"}}") == Some(true)
        && q[24..32] == [0x0d, 0xe0, 0xb6, 0xb3, 0xa7, 0x64, 0x00, 0x00]
}
