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

//! What a resumable load is holding.

use alloc::vec::Vec;

use super::super::error::BlkError;
use super::super::store::StoreEntry;
use super::super::store_toc::TocEntry;

/// Where a load has got to. The table of contents is read once at the start;
/// everything after that is walking it.
pub struct Load {
    pub(super) toc: Vec<TocEntry>,
    /// Entry being read.
    pub(super) idx: usize,
    /// Bytes of that entry read so far.
    pub(super) data: Vec<u8>,
    /// Entries finished and verified.
    pub(super) staged: Vec<StoreEntry>,
}

pub enum Step {
    /// More work remains. Call again on the next idle slot.
    More,
    /// Everything read and verified.
    Done(Vec<StoreEntry>),
    /// The container is unreadable. The caller records the class and stops:
    /// retrying a corrupt table of contents produces the same answer slower.
    Failed(BlkError),
}
