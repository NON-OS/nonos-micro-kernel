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

//! Reading a commit's content bytes back into a record.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::oid::ObjectId;

use super::signature::Signature;
use super::types::Commit;

/// Why a byte slice is not a well-formed commit.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CommitError {
    /// No `tree` line, or it did not come first.
    Tree,
    /// A `parent` line held something that is not an object id.
    Parent,
    /// An `author` or `committer` line was missing or unparseable.
    Signature,
    /// The header was not terminated by a blank line.
    Header,
    /// The message was not valid UTF-8.
    Message,
}

/// Parse commit content. Headers are read in the order git writes them, so a
/// commit whose lines are shuffled is refused rather than silently accepted
/// into a different id than the bytes imply.
pub fn parse(content: &[u8]) -> Result<Commit, CommitError> {
    let mut lines = Lines::new(content);

    let tree_line = lines.next().ok_or(CommitError::Tree)?;
    let tree_hex = strip(tree_line, b"tree ").ok_or(CommitError::Tree)?;
    let tree = oid(tree_hex).ok_or(CommitError::Tree)?;

    let mut parents = Vec::new();
    let mut line = lines.next().ok_or(CommitError::Signature)?;
    while let Some(hex) = strip(line, b"parent ") {
        parents.push(oid(hex).ok_or(CommitError::Parent)?);
        line = lines.next().ok_or(CommitError::Signature)?;
    }

    let author_line = strip(line, b"author ").ok_or(CommitError::Signature)?;
    let author = Signature::parse(author_line).ok_or(CommitError::Signature)?;

    let committer_line = lines.next().ok_or(CommitError::Signature)?;
    let committer_line = strip(committer_line, b"committer ").ok_or(CommitError::Signature)?;
    let committer = Signature::parse(committer_line).ok_or(CommitError::Signature)?;

    // Extra headers git may write, such as `encoding` or a gpg signature, run
    // until the blank line. They are not carried, and a commit holding them is
    // not re-encoded, only read.
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

/// A cursor over newline-separated header lines that can hand back the
/// remaining bytes verbatim once the header ends.
struct Lines<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Lines<'a> {
    fn new(data: &'a [u8]) -> Lines<'a> {
        Lines { data, pos: 0 }
    }

    fn next(&mut self) -> Option<&'a [u8]> {
        if self.pos > self.data.len() {
            return None;
        }
        if self.pos == self.data.len() {
            self.pos += 1;
            return Some(&self.data[self.data.len()..]);
        }
        let end = self.data[self.pos..]
            .iter()
            .position(|b| *b == b'\n')
            .map(|p| p + self.pos)
            .unwrap_or(self.data.len());
        let line = &self.data[self.pos..end];
        self.pos = end + 1;
        Some(line)
    }

    fn rest(&self) -> &'a [u8] {
        if self.pos >= self.data.len() {
            &self.data[self.data.len()..]
        } else {
            &self.data[self.pos..]
        }
    }
}

fn strip<'a>(line: &'a [u8], prefix: &[u8]) -> Option<&'a [u8]> {
    if line.len() >= prefix.len() && &line[..prefix.len()] == prefix {
        Some(&line[prefix.len()..])
    } else {
        None
    }
}

fn oid(hex: &[u8]) -> Option<ObjectId> {
    ObjectId::from_hex(core::str::from_utf8(hex).ok()?)
}
