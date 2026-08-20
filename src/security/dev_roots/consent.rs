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

use super::pending::PENDING;

/// Six digits, so it is short enough to read off a screen and retype without
/// error, and long enough that one guess is worth a millionth.
const CHALLENGE_MODULUS: u32 = 1_000_000;

/// Begin an enrolment and show the user a code only the trusted path can
/// display.
///
/// This is the whole consent mechanism, and it rests on one property: the
/// challenge is written by the kernel straight to the console, which no
/// capsule can read. A capsule may ask to enrol a root, but it cannot learn
/// the number needed to complete it, so the confirmation can only come from
/// somebody looking at the machine.
///
/// Weaker designs put the decision in a window drawn by userspace. That only
/// moves the trust into whichever process draws the window, and a compromised
/// shell then approves silently on the user's behalf.
pub(super) fn arm_challenge(root: [u8; 32]) -> u32 {
    let challenge = crate::crypto::random::secure_random_u32() % CHALLENGE_MODULUS;
    PENDING.lock().arm(root, challenge);

    crate::sys::serial::println(b"");
    crate::sys::serial::println(b"  == DEVELOPER ROOT ENROLMENT REQUESTED ==");
    crate::sys::serial::println(b"  Software built on this machine wants permission to run here.");
    crate::sys::serial::println(b"  Anything proved under this root will be allowed to start.");
    crate::sys::serial::print(b"  Confirmation code: ");
    crate::sys::serial::print_dec(challenge as u64);
    crate::sys::serial::println(b"");
    crate::sys::serial::println(b"  Type this code to approve. Ignore it if you did not ask.");
    crate::sys::serial::println(b"");
    challenge
}

/// Complete an enrolment if `answer` matches the displayed code.
pub(super) fn redeem(answer: u32) -> Option<[u8; 32]> {
    PENDING.lock().redeem(answer)
}
