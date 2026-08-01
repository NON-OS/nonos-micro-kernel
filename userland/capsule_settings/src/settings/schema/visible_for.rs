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

// Appearance first, then pointer and keyboard, then locale and clock, then the
// two notification switches. Every field the policy service carries now
// appears on exactly one tab: thirteen were fetched on every launch and shown
// nowhere, so they could not be read or changed from the desktop at all.
const GENERAL_FIELDS: &[Field] = &[
    Field::Brightness,
    Field::Theme,
    Field::Wallpaper,
    Field::HighContrast,
    Field::FontSize,
    Field::AnimationsEnabled,
    Field::CursorSize,
    Field::MouseSensitivity,
    Field::KeyboardLayout,
    Field::Language,
    Field::Timezone,
    Field::ClockFormat24,
    Field::ScreenTimeout,
    Field::SoundEnabled,
    Field::NotificationsEnabled,
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
    Field::SystemKeysGenerated,
    Field::DeveloperMode,
    Field::KernelAslr,
    Field::KernelNxBit,
    Field::KernelSmep,
    Field::KernelSmap,
    Field::KernelSeccomp,
    Field::KernelStackGuard,
    Field::KernelIommu,
    Field::KernelWatchdog,
    Field::KernelPreempt,
    Field::KernelHugepages,
    Field::KernelDebug,
    Field::KernelSerial,
];

pub fn visible_for(category: Category) -> &'static [Field] {
    match category {
        Category::User => GENERAL_FIELDS,
        Category::Identity => NETWORK_FIELDS,
        Category::Kernel => SECURITY_FIELDS,
    }
}

// Every field the panel fetches has to be on a tab. Adding one to ALL_FIELDS
// and forgetting the tab is what left thirteen of them unreachable, and it
// costs nothing to catch that here instead.
const _: () = assert!(
    GENERAL_FIELDS.len() + NETWORK_FIELDS.len() + SECURITY_FIELDS.len()
        == super::all_fields::ALL_FIELDS.len(),
    "every policy field must appear on exactly one settings tab"
);
