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

//! How a kernel caller waits for a capsule's reply.

/// Rounds of plain yielding first: a capsule that outranks the caller runs
/// on the first of them, so the common case costs no sleep.
const FAST_ROUNDS: u32 = 64;

/// How long a caller that has fallen back to sleeping keeps waiting.
const SLOW_BUDGET_MS: u64 = 5_000;

/*
 * A yield only hands the CPU to something of higher priority: when the
 * capsule ranks below the caller, the scheduler picks the caller again and
 * fifty thousand yields pass in no time while the capsule never runs. A
 * block write from vfs failed that way, then landed on disk after the
 * kernel had already reported it lost. Sleeping takes the caller off the
 * run queue, so the capsule runs. The reply goes to a kernel inbox that
 * wakes no one, so each sleep is a single tick.
 */
/// Wait one round for a reply. False once the budget is spent.
pub(super) fn pause(round: u32, started_ms: u64) -> bool {
    if round >= FAST_ROUNDS
        && crate::time::timestamp_millis().saturating_sub(started_ms) >= SLOW_BUDGET_MS
    {
        return false;
    }
    rest(round);
    true
}

/// Give the CPU away for one round, by sleeping a tick once plain yields
/// have had their chance. A caller with no process can only yield.
pub(super) fn rest(round: u32) {
    let pid = match crate::process::current_pid() {
        Some(pid) if round >= FAST_ROUNDS => pid,
        _ => {
            crate::sched::yield_now();
            return;
        }
    };
    let token = crate::sched::wake_token(pid);
    let wake = crate::time::timestamp_millis().saturating_add(1);
    crate::sched::sleep_until_unless_woken(pid, wake, token);
    crate::sched::yield_now();
}
