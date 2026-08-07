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
//! Where a remote repository lives.

extern crate alloc;

use alloc::string::String;

/// A parsed `https://host/owner/repo.git` remote.
pub struct Remote {
    pub host: String,
    /// The path of the repository, with no trailing slash.
    pub base: String,
}

impl Remote {
    /// Parse a remote URL. Only HTTPS is accepted: git over plain HTTP would
    /// let anyone on the path serve the objects, and this has no way to tell
    /// the user that happened.
    pub fn parse(url: &str) -> Option<Remote> {
        let rest = url.strip_prefix("https://")?;
        let slash = rest.find('/')?;
        let (host, path) = rest.split_at(slash);
        if host.is_empty() || host.len() > 253 || path.len() < 2 {
            return None;
        }
        let path = path.trim_end_matches('/');
        Some(Remote { host: String::from(host), base: String::from(path) })
    }

    /// The name a clone gives the directory it creates.
    pub fn name(&self) -> &str {
        let tail = self.base.rsplit('/').next().unwrap_or("");
        tail.strip_suffix(".git").unwrap_or(tail)
    }
}
