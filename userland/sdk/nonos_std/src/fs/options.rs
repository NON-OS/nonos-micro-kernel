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

use super::file::{open_with, File};
use super::frame::{O_CREATE, O_TRUNC};
use crate::io::Result;

#[derive(Clone, Copy, Debug, Default)]
pub struct OpenOptions {
    create: bool,
    truncate: bool,
}

impl OpenOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(&mut self, _read: bool) -> &mut Self {
        self
    }

    pub fn write(&mut self, _write: bool) -> &mut Self {
        self
    }

    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }

    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }

    pub fn open(&self, path: &[u8]) -> Result<File> {
        let mut flags = 0;
        if self.create {
            flags |= O_CREATE;
        }
        if self.truncate {
            flags |= O_TRUNC;
        }
        open_with(path, flags)
    }
}
