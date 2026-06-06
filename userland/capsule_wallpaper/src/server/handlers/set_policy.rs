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

use crate::protocol::{read_u32, Request, E_INVAL, SET_POLICY_REQ_LEN};
use crate::server::respond;
use crate::state::{Context, Policy};

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != SET_POLICY_REQ_LEN {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let Some(raw) = read_u32(body, 0) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let Some(policy) = Policy::from_u32(raw) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    ctx.set_policy(policy);
    let _ = respond::status(sender_pid, req, 0, tx);
}
