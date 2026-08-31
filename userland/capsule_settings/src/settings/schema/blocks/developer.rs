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

pub const DEVELOPER: &[Block] = &[
    Block {
        title: "Developer mode",
        note: Some("Unverified capsules can never spawn in a production build."),
        pill: Pill::None,
        rows: &[Row::Field(Field::DeveloperMode)],
    },
    Block {
        title: "Diagnostics",
        note: None,
        pill: Pill::None,
        rows: &[Row::Field(Field::KernelDebug), Row::Field(Field::KernelSerial)],
    },
    Block {
        title: "Scheduler and memory",
        note: None,
        pill: Pill::None,
        rows: &[Row::Field(Field::KernelPreempt), Row::Field(Field::KernelHugepages)],
    },
];
