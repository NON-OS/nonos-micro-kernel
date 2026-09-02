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
use nonos_libc::mk_time_millis;

use crate::command;
use crate::jobs;
use crate::term::context::context_line;
use crate::term::cwd::home_var;
use crate::term::dimensions::COLS;
use crate::term::identity::{hostname, USER};
use crate::term::prompt::PROMPT_BYTES;
use crate::term::state::State;
use crate::term::util::{copy_into, format_u64};

pub fn on_enter(state: &mut State) -> EventOutcome {
    // Running a line ends any search that found it. The match is already on
    // the line, so accepting it is simply leaving the mode.
    super::search::search_accept(state);
    state.fresh = false;
    let started = mk_time_millis();
    state.open_block(crate::term::rtc::rtc_hms());
    let mut ctx = [0u8; COLS];
    let cn = context_line(USER, hostname(), state.cwd.as_bytes(), home_var(state), &mut ctx);
    state.scrollback.push_line(&ctx[..cn]);
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
    let mut prev_status: i32 = state.last_status;
    for command::Stmt { conn, body, background } in command::split_program(&entered[..n]) {
        let go = match conn {
            command::Conn::Always => true,
            command::Conn::And => prev_status == 0,
            command::Conn::Or => prev_status != 0,
        };
        if !go {
            continue;
        }
        let aliased = command::alias_expand(body, &state.aliases);
        let expanded = command::expand(&aliased, &state.vars, prev_status);
        state.last_status = 0;
        let argv = command::parse(&expanded);
        let args = &argv.argv[..argv.argc];
        match jobs::is_job_command(state, args) {
            jobs::Verdict::Job(work) => {
                let id = jobs::submit(state, body, background, work);
                if background {
                    print_started(state, id);
                    prev_status = state.last_status;
                    continue;
                }
                state.fg_running = true;
                state.fg_started_ms = mk_time_millis();
                break;
            }
            jobs::Verdict::Handled => {
                prev_status = state.last_status;
                continue;
            }
            jobs::Verdict::Instant => {}
        }
        if let command::Outcome::Exit = command::run(state, &argv) {
            outcome = command::Outcome::Exit;
            break;
        }
        if state.fg_running {
            break;
        }
        prev_status = state.last_status;
    }
    if !state.fg_running {
        let dur = (mk_time_millis() - started).clamp(0, u32::MAX as i64) as u32;
        state.close_block(state.last_status == 0, dur);
    }
    state.evict_blocks();
    state.line.clear();
    state.scrollback.jump_bottom();
    match outcome {
        command::Outcome::Exit => EventOutcome::Close,
        command::Outcome::Repaint => EventOutcome::Repaint,
    }
}

// "[n] started" line printed when a background job is submitted; the
// job's own output streams into the scrollback as Task 13's on_tick pump
// steps it.
fn print_started(state: &mut State, id: u32) {
    let mut num = [0u8; 20];
    let nk = format_u64(id as u64, &mut num);
    let mut msg = [0u8; 32];
    let mut mk = 0;
    msg[mk] = b'[';
    mk += 1;
    mk += copy_into(&mut msg[mk..], &num[..nk]);
    mk += copy_into(&mut msg[mk..], b"] started");
    state.scrollback.push_line(&msg[..mk]);
}
