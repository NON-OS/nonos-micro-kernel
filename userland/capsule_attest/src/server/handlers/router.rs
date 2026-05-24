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

use super::{health, proof_boot, proof_capsule_list, proof_invariants, proof_summary};
use crate::protocol::{
    parse, E_BAD_OP, OP_HEALTHCHECK, OP_PROOF_BOOT, OP_PROOF_CAPSULE_LIST, OP_PROOF_INVARIANTS,
    OP_PROOF_SUMMARY,
};
use crate::server::respond;

pub fn route(input: &[u8], out: &mut [u8]) -> usize {
    let (req, _payload) = match parse(input) {
        Ok(parsed) => parsed,
        Err((req, status)) => return respond::status(out, &req, status),
    };
    match req.op {
        OP_HEALTHCHECK => health::run(out, &req),
        OP_PROOF_SUMMARY => proof_summary::run(out, &req),
        OP_PROOF_INVARIANTS => proof_invariants::run(out, &req),
        OP_PROOF_BOOT => proof_boot::run(out, &req),
        OP_PROOF_CAPSULE_LIST => proof_capsule_list::run(out, &req),
        _ => respond::status(out, &req, E_BAD_OP),
    }
}
