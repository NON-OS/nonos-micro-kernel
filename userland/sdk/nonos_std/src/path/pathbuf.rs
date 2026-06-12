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

use alloc::string::String;

use super::repr::Path;

pub struct PathBuf {
    inner: String,
}

impl PathBuf {
    pub fn new() -> Self {
        Self { inner: String::new() }
    }

    pub fn push<S: AsRef<str>>(&mut self, segment: S) {
        let seg = segment.as_ref();
        if seg.starts_with('/') {
            self.inner.clear();
        } else if !self.inner.is_empty() && !self.inner.ends_with('/') {
            self.inner.push('/');
        }
        self.inner.push_str(seg);
    }

    pub fn as_path(&self) -> &Path {
        Path::new(&self.inner)
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

impl Default for PathBuf {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: AsRef<str>> From<S> for PathBuf {
    fn from(s: S) -> Self {
        Self { inner: String::from(s.as_ref()) }
    }
}
