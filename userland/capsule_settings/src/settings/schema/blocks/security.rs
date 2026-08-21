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

use crate::settings::schema::rows::{Block, Pill, Row, Tone};

pub const SECURITY: &[Block] = &[
    Block {
        title: "Lock screen",
        note: None,
        pill: Pill::None,
        rows: &[Row::Field(Field::AutoLockTimeout), Row::Field(Field::AutoWipe)],
    },
    Block {
        title: "Attestation and keys",
        note: Some("Groth16 over BLS12-381, checked before any capsule spawns."),
        pill: Pill::Fixed("Enforced", Tone::Ok),
        rows: &[
            Row::Field(Field::HardwareCrypto),
            Row::Field(Field::ZkAttestation),
            Row::Field(Field::SystemKeysGenerated),
        ],
    },
    Block {
        title: "Kernel hardening",
        note: None,
        pill: Pill::None,
        rows: &[
            Row::Field(Field::KernelAslr),
            Row::Field(Field::KernelNxBit),
            Row::Field(Field::KernelSmep),
            Row::Field(Field::KernelSmap),
            Row::Field(Field::KernelStackGuard),
            Row::Field(Field::KernelSeccomp),
            Row::Field(Field::KernelIommu),
            Row::Field(Field::KernelWatchdog),
        ],
    },
];
