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

use super::pathbuf::PathBuf;

#[repr(transparent)]
pub struct Path {
    inner: str,
}

impl Path {
    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> &Path {
        unsafe { &*(s.as_ref() as *const str as *const Path) }
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn is_absolute(&self) -> bool {
        self.inner.starts_with('/')
    }

    pub fn file_name(&self) -> Option<&str> {
        let trimmed = self.inner.trim_end_matches('/');
        if trimmed.is_empty() {
            return None;
        }
        Some(trimmed.rfind('/').map_or(trimmed, |i| &trimmed[i + 1..]))
    }

    pub fn parent(&self) -> Option<&Path> {
        let trimmed = self.inner.trim_end_matches('/');
        let i = trimmed.rfind('/')?;
        Some(Path::new(if i == 0 { "/" } else { &trimmed[..i] }))
    }

    pub fn extension(&self) -> Option<&str> {
        let name = self.file_name()?;
        let dot = name.rfind('.')?;
        if dot == 0 {
            None
        } else {
            Some(&name[dot + 1..])
        }
    }

    pub fn join<S: AsRef<str>>(&self, other: S) -> PathBuf {
        let mut buf = PathBuf::from(&self.inner);
        buf.push(other);
        buf
    }
}
