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

use nonos_policy_proto::{Category, Field};

const DISPLAY_FIELDS: &[Field] = &[
    Field::Brightness,
    Field::MouseSensitivity,
    Field::CursorSize,
    Field::HighContrast,
    Field::FontSize,
    Field::Theme,
    Field::Wallpaper,
    Field::ScreenTimeout,
    Field::AnimationsEnabled,
];

const NETWORK_FIELDS: &[Field] = &[
    Field::WifiAutoconnect,
    Field::AnonymousMode,
    Field::NymEnabled,
    Field::Hostname,
    Field::DomainName,
];

const SECURITY_FIELDS: &[Field] = &[
    Field::AutoLockTimeout,
    Field::AutoWipe,
    Field::HardwareCrypto,
    Field::ZkAttestation,
    Field::DeveloperMode,
    Field::KernelAslr,
    Field::KernelNxBit,
    Field::KernelSmep,
    Field::KernelSmap,
    Field::KernelSeccomp,
];

pub fn visible_for(category: Category) -> &'static [Field] {
    match category {
        Category::User => DISPLAY_FIELDS,
        Category::Identity => NETWORK_FIELDS,
        Category::Kernel => SECURITY_FIELDS,
    }
}
