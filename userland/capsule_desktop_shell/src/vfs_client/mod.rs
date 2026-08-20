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

//! Small client for the vfs_pool service. The desktop uses it to list the root
//! and to create files and folders in the same filesystem the file manager and
//! text editor see, speaking the wire protocol directly over IPC.

mod call;
mod classify;
mod constants;
mod create_file;
mod entry;
mod frame;
mod list;
mod mkdir;
mod owner_body;
mod parse;
mod path;
mod remove;
mod rename;
mod store_status;
mod under;
mod walk;

pub use create_file::create_file;
pub use entry::Entry;
pub use list::list;
pub use mkdir::mkdir;
pub use remove::remove;
pub use rename::rename;
pub use store_status::store_status;
