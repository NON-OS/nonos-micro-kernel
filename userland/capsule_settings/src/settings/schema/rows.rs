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

/// A value the panel reads from the running system rather than from policy.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Live {
    LinkState,
    IpAddress,
    Gateway,
    Dns,
    Adapter,
    Version,
    Commit,
    Toolchain,
    Architecture,
    StorageService,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Ok,
    Warn,
    Idle,
}

/// The badge on a card header. `Net` and `Radio` resolve against live state at
/// paint time; `Fixed` is a constant the table already knows.
#[derive(Clone, Copy)]
pub enum Pill {
    None,
    Net,
    Radio,
    Fixed(&'static str, Tone),
}

#[derive(Clone, Copy)]
pub enum Row {
    Field(Field),
    Live(&'static str, Live),
    Networks,
}

#[derive(Clone, Copy)]
pub struct Block {
    pub title: &'static str,
    pub note: Option<&'static str>,
    pub pill: Pill,
    pub rows: &'static [Row],
}
