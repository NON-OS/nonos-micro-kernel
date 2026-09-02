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

/// Saved project slots the record carries. Fixed so `Prefs` stays `Copy` and
/// the on-disk record stays a constant length.
pub const MAX_PROJECTS: usize = 8;
pub const PATH_CAP: usize = 48;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Project {
    pub len: u8,
    pub path: [u8; PATH_CAP],
}

impl Project {
    /// Truncates on a char boundary, so a path that overruns the slot still
    /// decodes as text instead of a broken code point.
    pub fn new(src: &[u8]) -> Self {
        let mut n = src.len().min(PATH_CAP);
        while n > 0 && n < src.len() && src[n] & 0xC0 == 0x80 {
            n -= 1;
        }
        let mut path = [0u8; PATH_CAP];
        path[..n].copy_from_slice(&src[..n]);
        Self { len: n as u8, path }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.path[..(self.len as usize).min(PATH_CAP)]
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(self.as_bytes()).unwrap_or("")
    }
}

impl Default for Project {
    fn default() -> Self {
        Self { len: 0, path: [0u8; PATH_CAP] }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Prefs {
    pub theme: u16,
    pub font_scale: u8,
    pub cursor: u8,
    pub rails: u8,
    pub projects: [Project; MAX_PROJECTS],
    pub project_count: u8,
}

impl Prefs {
    pub fn project_slice(&self) -> &[Project] {
        &self.projects[..(self.project_count as usize).min(MAX_PROJECTS)]
    }

    pub fn push_project(&mut self, path: &[u8]) -> bool {
        let n = (self.project_count as usize).min(MAX_PROJECTS);
        if n == MAX_PROJECTS || path.is_empty() {
            return false;
        }
        let slot = Project::new(path);
        if self.projects[..n].iter().any(|p| p.as_bytes() == slot.as_bytes()) {
            return false;
        }
        self.projects[n] = slot;
        self.project_count = (n + 1) as u8;
        true
    }
}

impl Default for Prefs {
    fn default() -> Self {
        Self {
            theme: 0,
            font_scale: 2,
            cursor: 0,
            rails: 0,
            projects: [Project::default(); MAX_PROJECTS],
            project_count: 0,
        }
    }
}
