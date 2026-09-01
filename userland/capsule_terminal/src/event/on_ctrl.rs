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

use nonos_app_skeleton::{EventOutcome, MOD_SHIFT};
use nonos_libc::mk_kill;

use super::accept_suggestion::accept_suggestion;
use super::copy_line::copy_line;
use super::paste_clipboard::paste_clipboard;
use super::search::{search_cancel, search_step};
use crate::jobs::JobWork;
use crate::term::state::State;

const SIGINT: u64 = 2;

const CTRL_A: u32 = 0x41;
const CTRL_C: u32 = 0x43;
const CTRL_E: u32 = 0x45;
const CTRL_K: u32 = 0x4B;
const CTRL_L: u32 = 0x4C;
const CTRL_R: u32 = 0x52;
const CTRL_R_LO: u32 = 0x72;
const CTRL_U: u32 = 0x55;
const CTRL_V: u32 = 0x56;
const CTRL_W: u32 = 0x57;
const CTRL_A_LO: u32 = 0x61;
const CTRL_C_LO: u32 = 0x63;
const CTRL_E_LO: u32 = 0x65;
const CTRL_K_LO: u32 = 0x6B;
const CTRL_L_LO: u32 = 0x6C;
const CTRL_U_LO: u32 = 0x75;
const CTRL_V_LO: u32 = 0x76;
const CTRL_W_LO: u32 = 0x77;

pub fn on_ctrl(state: &mut State, code: u32, flags: u16) -> Option<EventOutcome> {
    let shift = flags & MOD_SHIFT != 0;
    match code {
        CTRL_V | CTRL_V_LO => Some(paste_clipboard(state)),
        CTRL_C | CTRL_C_LO if shift => Some(copy_line(state)),
        // The same key starts a search and steps it, which is what every
        // shell binds it to: the key is pressed again because the match on
        // screen is not the one that was meant.
        CTRL_R | CTRL_R_LO => {
            search_step(state);
            Some(EventOutcome::Repaint)
        }
        // Leaving a search puts the line back as it was, rather than killing
        // a line the reader never chose to change.
        CTRL_C | CTRL_C_LO if state.search.is_some() => {
            search_cancel(state);
            Some(EventOutcome::Repaint)
        }
        CTRL_L | CTRL_L_LO => {
            state.scrollback.clear();
            state.scrollback.jump_bottom();
            Some(EventOutcome::Repaint)
        }
        CTRL_C | CTRL_C_LO => {
            if state.fg_running {
                if let Some(id) = state.jobs.foreground() {
                    if let Some(job) = state.jobs.get_mut(id) {
                        if let JobWork::ExternalStage { pid, .. } = job.work {
                            let _ = mk_kill(pid as u64, SIGINT);
                        }
                        job.cancel = true;
                    }
                }
            } else {
                state.line.clear();
                state.history.reset_cursor();
            }
            state.scrollback.push_line(b"^C");
            state.scrollback.jump_bottom();
            Some(EventOutcome::Repaint)
        }
        CTRL_U | CTRL_U_LO => {
            state.line.clear();
            Some(EventOutcome::Repaint)
        }
        CTRL_W | CTRL_W_LO => {
            state.line.delete_word();
            Some(EventOutcome::Repaint)
        }
        CTRL_K | CTRL_K_LO => {
            state.line.kill_to_end();
            Some(EventOutcome::Repaint)
        }
        CTRL_A | CTRL_A_LO => {
            state.line.move_home();
            Some(EventOutcome::Repaint)
        }
        CTRL_E | CTRL_E_LO => {
            if !accept_suggestion(state) {
                state.line.move_end();
            }
            Some(EventOutcome::Repaint)
        }
        // Ctrl+= / Ctrl++ zoom the body font in, Ctrl+- / Ctrl+_ out (1..=4).
        ZOOM_IN_EQ | ZOOM_IN_PLUS => {
            state.zoom_req += 1;
            Some(EventOutcome::Repaint)
        }
        ZOOM_OUT_MINUS | ZOOM_OUT_USCORE => {
            state.zoom_req -= 1;
            Some(EventOutcome::Repaint)
        }
        _ => None,
    }
}

const ZOOM_IN_EQ: u32 = 0x3D; // '='
const ZOOM_IN_PLUS: u32 = 0x2B; // '+'
const ZOOM_OUT_MINUS: u32 = 0x2D; // '-'
const ZOOM_OUT_USCORE: u32 = 0x5F; // '_'
