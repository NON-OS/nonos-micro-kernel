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

//! Ordering the table.

use super::keys::{Act, Binding};
use super::keys_group::Group;

pub const SORT: &[Binding] = &[
    Binding {
        key: b"c",
        codes: &[0x43, 0x63],
        label: b"by processor share",
        group: Group::Sort,
        act: Act::SortCpu,
    },
    Binding {
        key: b"m",
        codes: &[0x4D, 0x6D],
        label: b"by resident memory",
        group: Group::Sort,
        act: Act::SortMem,
    },
    Binding {
        key: b"n",
        codes: &[0x4E, 0x6E],
        label: b"by name",
        group: Group::Sort,
        act: Act::SortName,
    },
    Binding {
        key: b"p",
        codes: &[0x50, 0x70],
        label: b"by pid",
        group: Group::Sort,
        act: Act::SortPid,
    },
    Binding {
        key: b"i",
        codes: &[0x49, 0x69],
        label: b"by messages per second",
        group: Group::Sort,
        act: Act::SortIpc,
    },
    Binding {
        key: b"y",
        codes: &[0x59, 0x79],
        label: b"by syscalls per second",
        group: Group::Sort,
        act: Act::SortSysc,
    },
];
