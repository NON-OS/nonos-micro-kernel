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
//! The repository config file.
//!
//! Only what this needs: the remotes a clone records and a push reads back.
//! Git's config format is larger than this, so parsing keeps to sections and
//! `name = value` lines and ignores what it does not recognise rather than
//! rejecting a file git wrote.

mod parse;
mod read;
mod remote;
mod write;

pub use read::remote_url;
pub use remote::set_remote;
