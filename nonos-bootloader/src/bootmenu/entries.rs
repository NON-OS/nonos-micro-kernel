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

use crate::menu::{MenuAction, SecurityMode};

// Hardened is index 0 so it is both the highlighted default and the
// timeout selection. Development is intentionally absent: an unsigned,
// unattested boot is only reachable through the explicit dev override,
// never from this menu.
pub(super) const ENTRIES: [MenuAction; 6] = [
    MenuAction::Boot(SecurityMode::Hardened),
    MenuAction::Boot(SecurityMode::Standard),
    MenuAction::SafeMode,
    MenuAction::NetworkIsolated,
    MenuAction::Recovery,
    MenuAction::Shutdown,
];
