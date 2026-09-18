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

//! Every key this window answers to, written down once.
//!
//! The dispatcher used to match on bare hex, and nothing on screen said any of
//! these existed. Twenty shortcuts that only a reader of the source could find
//! are not features. The obvious fix is a help overlay, but a second list typed
//! out by hand would drift from the first the day someone rebinds a key, and a
//! help screen that lies is worse than none.
//!
//! So this is the list, and both the dispatcher and the overlay walk it. A key
//! that is not here does nothing; a key here that the dispatcher cannot act on
//! fails to compile.

pub use super::keys_group::Group;

/// What a key does, named so the dispatcher and the overlay agree on the word.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Act {
    Screen(usize),
    NextScreen,
    Security,
    Terminate,
    ForceKill,
    SortCpu,
    SortMem,
    SortName,
    SortPid,
    SortIpc,
    SortSysc,
    Refresh,
    FilterAll,
    FilterElevated,
    FilterProtected,
    FilterFlagged,
    Search,
    Help,
    Close,
}

/// One binding: the key as it is printed, the codes that trigger it, and the
/// sentence the overlay shows. The label is a verb phrase because the reader is
/// asking what the key will do, not what it is called.
pub struct Binding {
    pub key: &'static [u8],
    pub codes: &'static [u32],
    pub label: &'static [u8],
    pub group: Group,
    pub act: Act,
}
