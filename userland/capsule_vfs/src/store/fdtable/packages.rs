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
use alloc::vec::Vec;

use super::types::{File, Store, MAX_FILES};
use crate::blk::store::StoreEntry;

// Packages are staged from the block device rather than embedded, because
// whole-set enrollment derives every capsule's trailer from every capsule
// image: a trailer compiled into this binary changes the binary, which changes
// the root that produced the trailer, so an embedded copy can never converge.
// Which files exist is the host packer's decision; this side only places each
// payload at the absolute path its own descriptor carries. A device that is
// missing, wedged, or carrying a foreign container leaves /capsules empty
// instead of failing the capsule, since the rest of the filesystem still works.

impl Store {
    /// Take the entries a resumable load finished with.
    ///
    /// Split from the reading so the walk can be interrupted: `seed_packages`
    /// did both, which is why it held the receive loop for the whole container.
    pub(crate) fn adopt_staged(&mut self, staged: Vec<StoreEntry>) {
        for entry in staged {
            self.stage(entry.name, entry.data);
        }
        /*
         * Staging lands from the idle slot, not through dispatch, so the
         * counter the desktop and the package list wait on is moved here.
         */
        crate::server::generation::bump();
    }

    fn stage(&mut self, name: String, data: Vec<u8>) {
        if self.files.len() >= MAX_FILES || self.files.iter().any(|f| f.name == name) {
            return;
        }
        self.files.push(File::new(name, data, false));
    }
}
