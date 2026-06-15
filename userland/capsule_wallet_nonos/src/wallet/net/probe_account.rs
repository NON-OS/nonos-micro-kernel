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

use super::account_status::AccountStatus;
use super::constants::{SERVICE_DNS, SERVICE_SOCKETS};

pub fn probe_account(address: &[u8; 20]) -> AccountStatus {
    let dns = super::lookup::lookup(SERVICE_DNS);
    let sockets = super::lookup::lookup(SERVICE_SOCKETS);
    if dns == 0 || sockets == 0 {
        return AccountStatus::empty();
    }
    let mut out = AccountStatus::empty();
    fill_balance(&mut out, dns, sockets, address);
    fill_nonce(&mut out, dns, sockets, address);
    fill_fee(&mut out, dns, sockets);
    out
}

fn fill_balance(out: &mut AccountStatus, dns: u32, sockets: u32, address: &[u8; 20]) {
    let body = super::super::rpc::request_balance(address, 2);
    let Some(resp) = super::fetch_rpc::fetch_rpc(dns, sockets, &body) else { return };
    let Some(balance) = super::super::rpc::parse_quantity32(&resp) else { return };
    out.balance_wei = balance;
    out.balance_ready = true;
}

fn fill_nonce(out: &mut AccountStatus, dns: u32, sockets: u32, address: &[u8; 20]) {
    let body = super::super::rpc::request_nonce(address, 3);
    let Some(resp) = super::fetch_rpc::fetch_rpc(dns, sockets, &body) else { return };
    let Some(nonce) = super::super::rpc::parse_u64(&resp) else { return };
    out.nonce = nonce;
    out.nonce_ready = true;
}

fn fill_fee(out: &mut AccountStatus, dns: u32, sockets: u32) {
    let body = super::super::rpc::request_fee(4);
    let Some(resp) = super::fetch_rpc::fetch_rpc(dns, sockets, &body) else { return };
    let Some(fee) = super::super::rpc::parse_u64(&resp) else { return };
    out.fee_wei = fee;
    out.fee_ready = true;
}
