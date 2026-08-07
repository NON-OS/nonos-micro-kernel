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

//! How long to wait, given what is carrying the request.
//!
//! A direct socket answers in milliseconds, and a request that has gone quiet
//! for a few seconds has almost certainly failed. A mixnet holds every packet
//! at every hop on purpose, so the same exchange takes seconds, and the parts
//! of one answer arrive separated by more silence than a direct fetch would
//! ever survive.
//!
//! The same numbers cannot serve both. Tuned for the socket they cut a mixnet
//! handshake in half and call it a failure; tuned for the mixnet they leave a
//! broken direct fetch hanging for minutes. So the transport is asked.

use super::constants;
use crate::browser::net::mixnet;

/// Longest a whole fetch may take.
pub fn max_fetch_ms() -> i64 {
    if mixnet::is_on() {
        // A handshake alone is several round trips, and each one is paid for
        // in mixing. The direct budget expires before the certificate has
        // finished arriving.
        180_000
    } else {
        constants::MAX_FETCH_MS
    }
}

/// Quiet reads before an answer that has produced nothing is given up on.
pub fn first_wait() -> u32 {
    if mixnet::is_on() {
        240
    } else {
        constants::FIRST_WAIT
    }
}

/// Quiet reads before a body that has stopped arriving is taken as finished.
pub fn idle_after() -> u32 {
    if mixnet::is_on() {
        120
    } else {
        constants::IDLE_AFTER
    }
}

/// Quiet reads before a handshake flight that looks complete is believed.
///
/// This is the one that matters most. A flight looks settled as soon as one
/// whole record of it has arrived, and over the mixnet the rest of the same
/// flight can be seconds behind. Believing it early means verifying half a
/// certificate.
pub fn flight_settle() -> u32 {
    if mixnet::is_on() {
        90
    } else {
        constants::FLIGHT_SETTLE
    }
}

/// Quiet reads before a handshake is abandoned.
pub fn hs_wait() -> u32 {
    if mixnet::is_on() {
        400
    } else {
        constants::HS_WAIT
    }
}
