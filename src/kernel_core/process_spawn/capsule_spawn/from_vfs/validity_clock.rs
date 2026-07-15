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

/// 2020-01-01T00:00:00Z in Unix milliseconds. Before the wall clock is set from
/// the RTC or firmware it tracks uptime from zero and reads far below this, so a
/// value under this floor cannot be a real wall clock.
const MIN_PLAUSIBLE_UNIX_MS: u64 = 1_577_836_800_000;

/// Choose the timestamp a certificate validity window is enforced against. A
/// plausible wall clock passes through so `valid_from`/`valid_until` are
/// checked; an unset clock (uptime-since-boot, well below any real epoch) yields
/// `None`, so the signature and trust anchor still gate the load but the
/// temporal check is not falsely tripped into rejecting every certificate.
pub(crate) fn validity_now_ms(now: u64) -> Option<u64> {
    (now >= MIN_PLAUSIBLE_UNIX_MS).then_some(now)
}
