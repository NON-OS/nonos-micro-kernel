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

use crate::settings::section::Section;

use super::blocks::{
    APPEARANCE, DEVELOPER, GENERAL, NETWORK, PRIVACY, SECURITY, SOUND, STORAGE, UPDATES, WIFI,
};
use super::rows::Block;

pub const fn blocks_for(section: Section) -> &'static [Block] {
    match section {
        Section::General => GENERAL,
        Section::Network => NETWORK,
        Section::Wifi => WIFI,
        Section::Security => SECURITY,
        Section::Appearance => APPEARANCE,
        Section::Privacy => PRIVACY,
        Section::Sound => SOUND,
        Section::Storage => STORAGE,
        Section::Updates => UPDATES,
        Section::Developer => DEVELOPER,
    }
}
