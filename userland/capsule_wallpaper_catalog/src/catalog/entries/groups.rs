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

use super::featured::FEATURED;
use super::field_focus::FIELD_FOCUS;
use super::hardware_aesthetic::HARDWARE_AESTHETIC;
use super::network_topology::NETWORK_TOPOLOGY;
use super::special_variant::SPECIAL_VARIANT;
use crate::catalog::entry::Entry;

pub(crate) const ENTRY_GROUPS: &[&[Entry]] = &[
    FIELD_FOCUS,
    HARDWARE_AESTHETIC,
    NETWORK_TOPOLOGY,
    SPECIAL_VARIANT,
    // Appended last so it is a pure addition: every existing wallpaper keeps its
    // index and the default is unchanged. Pepe is the final entry in the picker.
    FEATURED,
];
