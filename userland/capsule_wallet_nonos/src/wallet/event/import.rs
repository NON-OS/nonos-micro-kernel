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

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ENTER, KEY_ESC};

use crate::wallet::ipc::{import_wallet, wallet_address};
use crate::wallet::state::{State, VIEW_RECEIVE};

// Open or close the private-key import field. Closing always wipes whatever was
// typed so a cancelled import leaves no key material behind.
pub fn toggle_import(state: &mut State) -> EventOutcome {
    state.import_active = !state.import_active;
    if state.import_active {
        state.view = VIEW_RECEIVE;
        state.status = b"paste a 0x private key, Enter to import";
    } else {
        wipe(state);
    }
    EventOutcome::Repaint
}

// Feed one key into the import field. Hex digits accumulate up to 64 nibbles;
// Enter submits, Esc cancels, Backspace deletes. The typed value is never
// rendered, only counted.
pub fn import_input(state: &mut State, code: u32) -> EventOutcome {
    if code == KEY_ESC {
        wipe(state);
        state.import_active = false;
        state.status = b"import cancelled";
        return EventOutcome::Repaint;
    }
    if code == KEY_ENTER {
        return submit(state);
    }
    if code == KEY_BACKSPACE {
        if state.import_len > 0 {
            state.import_len -= 1;
            state.import_hex[state.import_len] = 0;
        }
        return EventOutcome::Repaint;
    }
    // Accept a leading 0x without consuming a nibble slot.
    if state.import_len == 0 && (code == b'x' as u32 || code == b'X' as u32) {
        return EventOutcome::Repaint;
    }
    if super::hex_digit::hex_digit(code).is_some() && state.import_len < state.import_hex.len() {
        state.import_hex[state.import_len] = code as u8;
        state.import_len += 1;
    }
    EventOutcome::Repaint
}

fn submit(state: &mut State) -> EventOutcome {
    if state.import_len != 64 {
        state.status = b"private key must be 64 hex chars";
        return EventOutcome::Repaint;
    }
    let mut secret = [0u8; 32];
    for i in 0..32 {
        let hi = super::hex_digit::hex_digit(state.import_hex[i * 2] as u32);
        let lo = super::hex_digit::hex_digit(state.import_hex[i * 2 + 1] as u32);
        match (hi, lo) {
            (Some(h), Some(l)) => secret[i] = (h << 4) | l,
            _ => {
                zeroize(&mut secret);
                wipe(state);
                state.status = b"invalid private key";
                return EventOutcome::Repaint;
            }
        }
    }
    let outcome = match import_wallet(state.keyring_port, state.owner_pid, &secret) {
        Ok(id) => match wallet_address(state.keyring_port, state.owner_pid, id) {
            Ok(addr) => {
                state.wallet_id = id;
                state.address = addr;
                state.address_ready = true;
                state.import_active = false;
                state.status = b"wallet imported";
                super::probe_net::probe_net(state)
            }
            Err(_) => {
                state.status = b"address failed";
                EventOutcome::Repaint
            }
        },
        Err(_) => {
            state.status = b"import failed";
            EventOutcome::Repaint
        }
    };
    zeroize(&mut secret);
    wipe(state);
    outcome
}

fn wipe(state: &mut State) {
    for b in state.import_hex.iter_mut() {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    state.import_len = 0;
}

fn zeroize(secret: &mut [u8; 32]) {
    for b in secret.iter_mut() {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
}
