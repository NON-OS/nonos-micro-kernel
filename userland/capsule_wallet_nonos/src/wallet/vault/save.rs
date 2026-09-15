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

//! Writing the blob, and making it survive the power going off.
//!
//! Two steps, and the second is the one that matters. A write lands in the
//! RAM filesystem; `OP_STORE_PERSIST` copies it into the block store, which
//! is what the seeder reads back at the next boot. A save that skipped it
//! would look like it worked, right up until the reboot it exists for.

use super::path::{VAULT_DIR, VAULT_PATH};
use super::persist::persist;
use super::save_ops::{close, mkdir, open_created, write_all};

pub fn save_blob(blob: &[u8]) -> bool {
    mkdir(VAULT_DIR);
    let Some(fd) = open_created(VAULT_PATH) else {
        return false;
    };
    let wrote = write_all(fd, blob);
    close(fd);
    wrote && persist(VAULT_PATH)
}
