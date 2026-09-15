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

// A shim for the two enums `prefs.rs` needs from `fm::state`, kept byte
// identical to `capsule_file_manager/src/fm/state.rs`. The real `state.rs`
// cannot be included here: it pulls `preview.rs`, which calls the vfs syscall
// client, so the host build would need the whole capsule runtime. Same reason
// and same pattern as `vfs_store/time.rs`.

#[derive(Clone, Copy)]
pub enum SortMode {
    Name,
    Size,
    Date,
    Type,
}

/// How the current directory is presented: a grid of large icons (the default,
/// matching a modern desktop file manager) or a compact detail list.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ViewKind {
    Grid,
    List,
}
