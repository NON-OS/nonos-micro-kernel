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

/// Pick the first available (nonzero) clock source in priority order,
/// evaluating the lower-priority sources lazily: the value the bootloader
/// handoff supplied, then a previously calibrated value, then a freshly
/// computed one. Zero means "not available", so the fresh source (a fresh
/// TSC calibration or RTC read) runs only when the earlier ones are absent.
pub(super) fn pick_nonzero(
    handoff: u64,
    calibrated: impl FnOnce() -> u64,
    fresh: impl FnOnce() -> u64,
) -> u64 {
    if handoff != 0 {
        return handoff;
    }
    let c = calibrated();
    if c != 0 {
        c
    } else {
        fresh()
    }
}
