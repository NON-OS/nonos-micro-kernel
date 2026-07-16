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

// The open flags std collects before a File::open. The backend reads
// create/create_new/truncate/append when it builds the OP_OPEN request;
// read and write are kept for the Debug view and API completeness.

#[derive(Clone, Debug, Default)]
pub struct OpenOptions {
    pub(super) read: bool,
    pub(super) write: bool,
    pub(super) append: bool,
    pub(super) truncate: bool,
    pub(super) create: bool,
    pub(super) create_new: bool,
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions::default()
    }
    pub fn read(&mut self, read: bool) {
        self.read = read;
    }
    pub fn write(&mut self, write: bool) {
        self.write = write;
    }
    pub fn append(&mut self, append: bool) {
        self.append = append;
    }
    pub fn truncate(&mut self, truncate: bool) {
        self.truncate = truncate;
    }
    pub fn create(&mut self, create: bool) {
        self.create = create;
    }
    pub fn create_new(&mut self, create_new: bool) {
        self.create_new = create_new;
    }
}
