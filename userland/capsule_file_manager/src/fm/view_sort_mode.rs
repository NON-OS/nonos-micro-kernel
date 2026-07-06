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

use core::cmp::Ordering;

use super::entries::Entry;
use super::file_ext::ext;
use super::file_kind::kind_of;
use super::state::SortMode;
use super::view_sort_name::by_name;

pub fn by_mode(a: &Entry, b: &Entry, mode: SortMode) -> Ordering {
    match mode {
        SortMode::Name => by_name(a, b),
        SortMode::Size => b.size.unwrap_or(0).cmp(&a.size.unwrap_or(0)).then_with(|| by_name(a, b)),
        SortMode::Date => b.mtime.cmp(&a.mtime).then_with(|| by_name(a, b)),
        SortMode::Type => ext(&a.label).cmp(ext(&b.label)).then_with(|| by_name(a, b)),
    }
    .then_with(|| kind_of(a).cmp(&kind_of(b)))
}
