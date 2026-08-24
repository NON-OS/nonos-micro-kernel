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

//! What the probe found, held for the bring-up that runs after it.

use spin::Once;

use super::super::probe::UnitInfo;

static PROBED: Once<UnitInfo> = Once::new();

/// What the first remapping unit reported, or `None` if the probe never
/// reached one. Bring-up treats `None` as "there is nothing to program".
pub fn probed() -> Option<&'static UnitInfo> {
    PROBED.get()
}

/// Latch the probe result. Called once, from `init`, and only on the path
/// where the unit answered; a failed probe leaves this empty on purpose.
pub(super) fn record(info: UnitInfo) {
    PROBED.call_once(|| info);
}
