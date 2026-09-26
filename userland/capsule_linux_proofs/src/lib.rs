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


//! The shipped source, included and exercised.

extern crate alloc;

#[path = "../../capsule_linux/src/linux/wayland/wire.rs"]
pub mod wire;

#[path = "../../capsule_linux/src/linux/wayland/args.rs"]
pub mod args;

#[path = "../../capsule_linux/src/linux/file/resolve.rs"]
pub mod resolve;

#[path = "../../capsule_linux/src/linux/file/dirent.rs"]
pub mod dirent;

#[path = "../../capsule_linux/src/linux/file/meta/statbuf.rs"]
pub mod statbuf;

#[path = "../../capsule_linux/src/linux/call/spawn/exec_shebang.rs"]
pub mod exec_shebang;

pub mod image;

/// The installer's parsers, which read bytes fetched off a network.
pub mod install;

#[cfg(test)]
mod tests;
