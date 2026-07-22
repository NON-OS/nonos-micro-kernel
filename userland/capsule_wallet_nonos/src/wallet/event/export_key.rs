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

use crate::wallet::ipc::export_secret;
use crate::wallet::state::{State, VIEW_RECEIVE};

const HEX: &[u8; 16] = b"0123456789abcdef";

/// Reveal or hide the private key for backup. Revealing asks the keyring for the
/// key (owner-gated), formats it as `0x`-prefixed hex for display, and wipes the
/// raw secret immediately. Hiding wipes the on-screen copy, so the key never
/// outlives the moment the user is looking at it.
pub fn toggle_export(state: &mut State) -> EventOutcome {
    if state.export_active {
        wipe(state);
        state.export_active = false;
        state.status = b"private key hidden";
        return EventOutcome::Repaint;
    }
    if !state.address_ready {
        state.status = b"generate or import a wallet first";
        return EventOutcome::Repaint;
    }
    match export_secret(state.keyring_port, state.owner_pid, state.wallet_id) {
        Ok(mut secret) => {
            state.export_hex[0] = b'0';
            state.export_hex[1] = b'x';
            for i in 0..32 {
                state.export_hex[2 + i * 2] = HEX[(secret[i] >> 4) as usize];
                state.export_hex[2 + i * 2 + 1] = HEX[(secret[i] & 0x0F) as usize];
            }
            for b in secret.iter_mut() {
                unsafe { core::ptr::write_volatile(b, 0) };
            }
            state.export_active = true;
            state.view = VIEW_RECEIVE;
            state.status = b"private key shown, never share it";
            EventOutcome::Repaint
        }
        Err(_) => {
            state.status = b"export failed";
            EventOutcome::Repaint
        }
    }
}

fn wipe(state: &mut State) {
    for b in state.export_hex.iter_mut() {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
}
