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

use crate::wallet::ipc::{recover_wallet, wallet_address};
use crate::wallet::state::{State, VIEW_RECEIVE};

// Recovery-phrase entry. Words are typed in the clear so the user can check
// them (a phrase is only ever typed to RECOVER, on a machine the user already
// trusts with the resulting key), wiped on submit or cancel. Each word is
// resolved against the BIP39 list locally for immediate feedback; the
// keyring re-validates the checksum before deriving anything.

pub fn toggle_recover(state: &mut State) -> EventOutcome {
    state.recover_active = !state.recover_active;
    if state.recover_active {
        state.view = VIEW_RECEIVE;
        state.import_active = false;
        state.status = b"type your 12-24 word phrase, Enter to recover";
    } else {
        wipe(state);
    }
    EventOutcome::Repaint
}

pub fn recover_input(state: &mut State, code: u32) -> EventOutcome {
    if code == KEY_ESC {
        wipe(state);
        state.recover_active = false;
        state.status = b"recovery cancelled";
        return EventOutcome::Repaint;
    }
    if code == KEY_ENTER {
        return submit(state);
    }
    if code == KEY_BACKSPACE {
        if state.recover_len > 0 {
            state.recover_len -= 1;
            // SAFETY: volatile write so removed characters do not linger.
            unsafe { core::ptr::write_volatile(&mut state.recover_buf[state.recover_len], 0) };
        }
        return EventOutcome::Repaint;
    }
    let ch = code as u8;
    let lower = ch.to_ascii_lowercase();
    let is_letter = lower.is_ascii_lowercase();
    let is_space = ch == b' ';
    if (is_letter || is_space) && state.recover_len < state.recover_buf.len() {
        // Collapse doubled separators so word splitting stays unambiguous.
        if is_space && (state.recover_len == 0 || state.recover_buf[state.recover_len - 1] == b' ')
        {
            return EventOutcome::Idle;
        }
        state.recover_buf[state.recover_len] = if is_space { b' ' } else { lower };
        state.recover_len += 1;
    }
    EventOutcome::Repaint
}

fn submit(state: &mut State) -> EventOutcome {
    let mut indices = [0u16; 24];
    let mut count = 0usize;
    {
        let typed = &state.recover_buf[..state.recover_len];
        for word in typed.split(|&b| b == b' ').filter(|w| !w.is_empty()) {
            if count == 24 {
                count = usize::MAX;
                break;
            }
            match nonos_hd::bip39::word_index(word) {
                Some(idx) => {
                    indices[count] = idx;
                    count += 1;
                }
                None => {
                    state.status = b"a word is not in the BIP39 list";
                    return EventOutcome::Repaint;
                }
            }
        }
    }
    if !matches!(count, 12 | 15 | 18 | 21 | 24) {
        state.status = b"phrase must be 12, 15, 18, 21 or 24 words";
        return EventOutcome::Repaint;
    }

    let outcome = match recover_wallet(state.keyring_port, state.owner_pid, &indices[..count]) {
        Ok(id) => match wallet_address(state.keyring_port, state.owner_pid, id) {
            Ok(addr) => {
                state.wallet_id = id;
                state.address = addr;
                state.address_ready = true;
                state.recover_active = false;
                state.status = b"wallet recovered";
                super::probe_tick::probe_kick(state)
            }
            Err(_) => {
                state.status = b"address failed";
                EventOutcome::Repaint
            }
        },
        Err(_) => {
            state.status = b"phrase rejected: checksum does not match";
            EventOutcome::Repaint
        }
    };
    for w in indices.iter_mut() {
        // SAFETY: volatile write so the parsed phrase does not linger.
        unsafe { core::ptr::write_volatile(w, 0) };
    }
    wipe(state);
    outcome
}

fn wipe(state: &mut State) {
    for b in state.recover_buf.iter_mut() {
        // SAFETY: volatile write so the typed phrase is really gone.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    state.recover_len = 0;
}
