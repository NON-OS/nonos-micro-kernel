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

// NONOS_NODESKTOP=1 builds bring up drivers and the input stack but leave
// the compositor, splash and shell unspawned, so the on-screen kernel log
// stays visible for the whole session. This is the hardware bring-up
// channel: one photo shows every driver that initialized and where a fault
// lands, with nothing painting over it and no serial cable required.
pub(super) fn desktop_enabled() -> bool {
    option_env!("NONOS_NODESKTOP").is_none()
}
