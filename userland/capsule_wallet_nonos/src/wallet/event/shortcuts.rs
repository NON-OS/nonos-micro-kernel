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

use crate::wallet::state::{
    State, VIEW_HOME, VIEW_NOX, VIEW_PROOF, VIEW_RECEIVE, VIEW_SEND, VIEW_SHIELDED, VIEW_SWAP,
};

// Keys that act wherever the reader is. Reached only after the field on the
// current view has declined the key, so a digit typed into an amount is an
// amount and a digit typed anywhere else is a jump.
pub fn shortcut(state: &mut State, code: u32) -> EventOutcome {
    match code {
        code if code == b'i' as u32 || code == b'I' as u32 => super::import::toggle_import(state),
        code if code == b'm' as u32 || code == b'M' as u32 => super::recover::toggle_recover(state),
        code if code == b'k' as u32 || code == b'K' as u32 => {
            super::export_key::toggle_export(state)
        }
        code if code == b'h' as u32 => view(state, VIEW_HOME),
        code if code == b'v' as u32 => view(state, VIEW_RECEIVE),
        code if code == b's' as u32 => view(state, VIEW_SEND),
        code if code == b'p' as u32 => view(state, VIEW_PROOF),
        code if code == b'1' as u32 => view(state, VIEW_HOME),
        code if code == b'2' as u32 => view(state, VIEW_RECEIVE),
        code if code == b'3' as u32 => view(state, VIEW_SEND),
        code if code == b'4' as u32 => view(state, VIEW_PROOF),
        code if code == b'5' as u32 => view(state, VIEW_SHIELDED),
        code if code == b'6' as u32 => view(state, VIEW_NOX),
        code if code == b'7' as u32 => view(state, VIEW_SWAP),
        code if code == b'd' as u32 || code == b'D' as u32 => view(state, VIEW_SHIELDED),
        code if code == b'x' as u32 || code == b'X' as u32 => view(state, VIEW_NOX),
        code if code == b'g' as u32 || code == b'G' as u32 => super::generate::generate(state),
        code if code == b'E' as u32 => super::sign_eth::sign_eth(state),
        code if code == b'n' as u32 || code == b'N' as u32 => super::sign_nox::sign_nox(state),
        code if code == b'P' as u32 => super::sign_both::sign_both(state),
        code if code == b'b' as u32 || code == b'B' as u32 => super::broadcast::broadcast(state),
        code if code == b'w' as u32 || code == b'W' as u32 => super::probe_tick::probe_kick(state),
        _ => EventOutcome::Idle,
    }
}

fn view(state: &mut State, view: u8) -> EventOutcome {
    state.view = view;
    EventOutcome::Repaint
}
