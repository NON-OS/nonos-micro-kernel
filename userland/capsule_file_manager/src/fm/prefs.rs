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

use alloc::{string::String, vec::Vec};

use nonos_app_skeleton::sidecar;

use super::state::{SortMode, ViewKind};

pub struct Prefs {
    pub view: ViewKind,
    pub sort: SortMode,
    pub sidebar_open: bool,
}

impl Default for Prefs {
    fn default() -> Self {
        Prefs { view: ViewKind::Grid, sort: SortMode::Name, sidebar_open: true }
    }
}

impl Prefs {
    // A missing or unparseable value falls back to the built-in default:
    // preferences must never be able to fail the app's startup.
    pub fn from_blob(buf: &[u8]) -> Prefs {
        let records = sidecar::decode(buf);
        let get = |k: &str| records.iter().find(|(key, _)| key == k).map(|(_, v)| v.as_str());
        let mut prefs = Prefs::default();
        if get("view") == Some("list") {
            prefs.view = ViewKind::List;
        }
        prefs.sort = match get("sort") {
            Some("size") => SortMode::Size,
            Some("date") => SortMode::Date,
            Some("type") => SortMode::Type,
            _ => SortMode::Name,
        };
        prefs.sidebar_open = get("sidebar") != Some("closed");
        prefs
    }

    pub fn to_blob(&self) -> Vec<u8> {
        let view = if self.view == ViewKind::List { "list" } else { "grid" };
        let sort = match self.sort {
            SortMode::Name => "name",
            SortMode::Size => "size",
            SortMode::Date => "date",
            SortMode::Type => "type",
        };
        let sidebar = if self.sidebar_open { "open" } else { "closed" };
        let records: Vec<(String, String)> = alloc::vec![
            (String::from("sidebar"), String::from(sidebar)),
            (String::from("sort"), String::from(sort)),
            (String::from("view"), String::from(view)),
        ];
        sidecar::encode(&records)
    }
}
