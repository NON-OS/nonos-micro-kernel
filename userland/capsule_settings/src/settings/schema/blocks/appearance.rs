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

pub const APPEARANCE: &[Block] = &[
    Block {
        title: "Theme",
        note: None,
        pill: Pill::None,
        rows: &[
            Row::Field(Field::Theme),
            Row::Field(Field::Wallpaper),
            Row::Field(Field::HighContrast),
        ],
    },
    Block {
        title: "Display",
        note: None,
        pill: Pill::None,
        rows: &[
            Row::Field(Field::Brightness),
            Row::Field(Field::FontSize),
            Row::Field(Field::AnimationsEnabled),
        ],
    },
    Block {
        title: "Pointer",
        note: None,
        pill: Pill::None,
        rows: &[Row::Field(Field::CursorSize), Row::Field(Field::MouseSensitivity)],
    },
];
