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

use nonos_policy_proto::Field;

use crate::settings::schema::rows::{Block, Pill, Row};

pub const GENERAL: &[Block] = &[
    Block {
        title: "Device",
        note: None,
        pill: Pill::None,
        rows: &[Row::Field(Field::Hostname), Row::Field(Field::DomainName)],
    },
    Block {
        title: "Language and region",
        note: None,
        pill: Pill::None,
        rows: &[
            Row::Field(Field::Language),
            Row::Field(Field::KeyboardLayout),
            Row::Field(Field::Timezone),
            Row::Field(Field::ClockFormat24),
        ],
    },
    Block {
        title: "Notifications",
        note: Some("Let capsules post toasts to the desktop shell."),
        pill: Pill::None,
        rows: &[Row::Field(Field::NotificationsEnabled)],
    },
];
