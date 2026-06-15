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

use super::constants::{SERVICE_DNS, SERVICE_SOCKETS};

pub fn broadcast_raw(raw: &[u8]) -> Option<[u8; 32]> {
    let dns = super::lookup::lookup(SERVICE_DNS);
    let sockets = super::lookup::lookup(SERVICE_SOCKETS);
    if dns == 0 || sockets == 0 {
        return None;
    }
    let body = super::super::rpc::request_broadcast(raw, 5);
    let resp = super::fetch_rpc::fetch_rpc(dns, sockets, &body)?;
    super::super::rpc::parse_hash32(&resp)
}
