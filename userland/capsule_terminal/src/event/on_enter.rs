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

use nonos_app_skeleton::EventOutcome;

use crate::command;
use crate::term::dimensions::COLS;
use crate::term::prompt::PROMPT_BYTES;
use crate::term::state::State;
use crate::term::util::copy_into;

pub fn on_enter(state: &mut State) -> EventOutcome {
    state.fresh = false;
    state.open_block(crate::term::rtc::rtc_hms());
    let body = state.line.as_bytes();
    let mut entered = [0u8; COLS];
    let n = body.len();
    entered[..n].copy_from_slice(body);
    let mut echo = [0u8; COLS + 8];
    let mut k = 0;
    k += copy_into(&mut echo[k..], PROMPT_BYTES);
    k += copy_into(&mut echo[k..], &entered[..n]);
    state.scrollback.push_line(&echo[..k]);
    state.history.push(&entered[..n]);
    let mut outcome = command::Outcome::Repaint;
    let mut prev_ok = true;
    for (conn, stmt) in command::split_program(&entered[..n]) {
        let go = match conn {
            command::Conn::Always => true,
            command::Conn::And => prev_ok,
            command::Conn::Or => !prev_ok,
        };
        if !go {
            continue;
        }
        state.last_status = true;
        let aliased = command::alias_expand(stmt, &state.aliases);
        let expanded = command::expand(&aliased, &state.vars);
        let argv = command::parse(&expanded);
        if let command::Outcome::Exit = command::run(state, &argv) {
            outcome = command::Outcome::Exit;
            break;
        }
        prev_ok = state.last_status;
    }
    state.close_block(state.last_status);
    state.evict_blocks();
    state.line.clear();
    state.scrollback.jump_bottom();
    match outcome {
        command::Outcome::Exit => EventOutcome::Close,
        command::Outcome::Repaint => EventOutcome::Repaint,
    }
}
