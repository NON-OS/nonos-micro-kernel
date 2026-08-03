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

//! The commit parser.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::commit::lines::Lines;
use crate::commit::sig::Signature;
use crate::commit::types::Commit;

use super::error::CommitError;
use super::field::{oid, strip};

/// Headers are read in the order git writes them, so a shuffled commit is
/// refused rather than accepted into a different id than its bytes imply.
/// Extra headers such as `encoding` or a gpg signature are skipped to the
/// blank line: they are read but never re-encoded.
pub fn parse(content: &[u8]) -> Result<Commit, CommitError> {
    let mut lines = Lines::new(content);

    let tree_line = lines.next().ok_or(CommitError::Tree)?;
    let tree =
        oid(strip(tree_line, b"tree ").ok_or(CommitError::Tree)?).ok_or(CommitError::Tree)?;

    let mut parents = Vec::new();
    let mut line = lines.next().ok_or(CommitError::Signature)?;
    while let Some(hex) = strip(line, b"parent ") {
        parents.push(oid(hex).ok_or(CommitError::Parent)?);
        line = lines.next().ok_or(CommitError::Signature)?;
    }

    let author = Signature::parse(strip(line, b"author ").ok_or(CommitError::Signature)?)
        .ok_or(CommitError::Signature)?;
    let c_line = lines.next().ok_or(CommitError::Signature)?;
    let committer = Signature::parse(strip(c_line, b"committer ").ok_or(CommitError::Signature)?)
        .ok_or(CommitError::Signature)?;

    loop {
        match lines.next() {
            Some([]) => break,
            Some(_) => continue,
            None => return Err(CommitError::Header),
        }
    }

    let message =
        core::str::from_utf8(lines.rest()).map(String::from).map_err(|_| CommitError::Message)?;
    Ok(Commit { tree, parents, author, committer, message })
}
