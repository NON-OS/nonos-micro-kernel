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


//! The service loop: take a trap from any thread of the guest, answer it
//! or leave the caller parked.

use nonos_libc::{mk_foreign_reply, mk_foreign_wait, ForeignFrame};

use super::answer::Answer;
use super::dispatch::answer;
use crate::linux::guest::Guest;

/// How long one wait blocks before looking at the guest again.
const WAIT_MS: u64 = 250;

pub fn serve(guest: &mut Guest) -> i32 {
    loop {
        let mut frame = ForeignFrame::default();
        let got = mk_foreign_wait(&mut frame, WAIT_MS);
        if got > 0 && guest.owns(frame.pid) {
            if let Answer::Reply(value) = answer(guest, &frame) {
                let _ = mk_foreign_reply(frame.pid, value);
            }
        }
        if let Some(code) = guest.exited {
            return code;
        }
    }
}
