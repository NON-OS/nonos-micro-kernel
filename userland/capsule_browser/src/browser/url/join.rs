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

use alloc::string::{String, ToString};

use super::authority::authority;
use super::has_scheme_prefix::has_scheme_prefix;
use super::path_without_fragment::path_without_fragment;
use super::path_without_query::path_without_query;
use super::types::{Scheme, Url};

pub fn join(base: &Url, location: &str) -> String {
    let loc = location.trim();
    if loc.starts_with("http://") || loc.starts_with("https://") {
        return loc.to_string();
    }
    if has_scheme_prefix(loc) {
        return loc.to_string();
    }
    let scheme = if base.scheme == Scheme::Https { "https" } else { "http" };
    if let Some(rest) = loc.strip_prefix("//") {
        return alloc::format!("{}://{}", scheme, rest);
    }
    if loc.starts_with('/') {
        return alloc::format!("{}://{}{}", scheme, authority(base), loc);
    }
    if loc.starts_with('?') {
        return alloc::format!(
            "{}://{}{}{}",
            scheme,
            authority(base),
            path_without_query(&base.path),
            loc
        );
    }
    if loc.starts_with('#') {
        return alloc::format!(
            "{}://{}{}{}",
            scheme,
            authority(base),
            path_without_fragment(&base.path),
            loc
        );
    }
    let dir = match base.path.rfind('/') {
        Some(i) => &base.path[..=i],
        None => "/",
    };
    alloc::format!("{}://{}{}{}", scheme, authority(base), dir, loc)
}
