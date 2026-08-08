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
//! Cloning a remote into a fresh repository.

extern crate alloc;

use crate::repo::{clone_into, CloneRequest};
use crate::storage::Storage;
use crate::transport::{Transport, TransportError};

use super::discover::{discover, UPLOAD_PACK};
use super::fetch::fetch;

/// Clone `branch` from the remote into `git_dir`, `depth` commits deep.
///
/// Returns the number of files written to the work tree. The head comes from
/// the remote's own advertisement, so a branch the remote does not have is an
/// error here rather than an empty repository the caller has to notice.
pub fn clone<T: Transport, S: Storage>(
    transport: &mut T,
    storage: &mut S,
    git_dir: &str,
    work_tree: &str,
    branch: &str,
    depth: u32,
) -> Result<usize, TransportError> {
    let refs = discover(transport, UPLOAD_PACK)?;
    let mut full = alloc::string::String::from("refs/heads/");
    full.push_str(branch);
    let head = refs.iter().find(|r| r.name == full).ok_or(TransportError::Malformed)?.id;

    let pack = fetch(transport, &[head], depth)?;
    let request = CloneRequest { git_dir, work_tree, head, branch, shallow: depth > 0 };
    clone_into(storage, &request, &pack).map_err(|_| TransportError::Malformed)
}
