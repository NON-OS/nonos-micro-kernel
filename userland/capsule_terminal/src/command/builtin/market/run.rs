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

use crate::command::output::Output;

use super::call_list::call_list;
use super::constants::{BODY_OFF, RSP_CAP};
use super::emit_call_failed::emit_call_failed;
use super::emit_count::emit_count;
use super::lookup_market::lookup_market;
use super::read_u32::read_u32;
use super::render_entries::render_entries;

pub fn run(out: &mut Output<'_>, _argv: &[&[u8]]) {
    let Some(port) = lookup_market() else {
        out.writeln(b"  market service not registered");
        return;
    };
    let mut rsp = [0u8; RSP_CAP];
    let n = match call_list(port, &mut rsp) {
        Ok(n) => n,
        Err(rc) => {
            emit_call_failed(out, rc);
            return;
        }
    };
    if n < BODY_OFF + 4 {
        out.writeln(b"  market: short reply");
        return;
    }
    let Some(count) = read_u32(&rsp[..n], BODY_OFF) else {
        out.writeln(b"  market: short reply");
        return;
    };
    emit_count(out, count);
    render_entries(out, &rsp[BODY_OFF + 4..n], count);
}
