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

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use super::entries::RESERVED_PREFIX;
use super::file_kind::kind_of_name;
use super::filetype::Kind;
use super::recents_filter::{keeps, KIND_ORDER};

/// Only the kinds the given paths actually contain, each with its real count.
/// A kind with no members contributes no chip, so a filter is never offered that
/// would empty the list it filters.
pub fn kind_counts<'a, I>(paths: I) -> Vec<(Kind, u32)>
where
    I: Iterator<Item = &'a str> + Clone,
{
    kind_tally(paths.map(kind_of_name))
}

/// The same tally over rows already classified, for a caller whose data says
/// more about a row than its path does -- a search hit knows it matched a
/// directory even when the path carries no trailing separator.
pub fn kind_tally<I: Iterator<Item = Kind> + Clone>(kinds: I) -> Vec<(Kind, u32)> {
    let mut out = Vec::new();
    for kind in KIND_ORDER {
        let n = kinds.clone().filter(|k| *k == kind).count() as u32;
        if n > 0 {
            out.push((kind, n));
        }
    }
    out
}

/// The journal rows the Recents surface will show: reserved sidecar paths are
/// dropped here, then the active chip is applied, so the chip counts and the
/// drawn list are computed from one pass.
pub fn shown(entries: &[(u64, String)], filter: Option<Kind>) -> Vec<(u64, &str)> {
    entries
        .iter()
        .map(|(ms, path)| (*ms, path.as_str()))
        .filter(|(_, path)| !path.starts_with(RESERVED_PREFIX))
        .filter(|(_, path)| keeps(filter, path))
        .collect()
}
