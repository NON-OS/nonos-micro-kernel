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

//! What an app is allowed to do, declared by the app itself.
//!
//! Every capability an app holds is written in its own source, expands into
//! the manifest it is signed and proved against, and is enforced by the
//! kernel at spawn. There is no set of powers an app gets for free beyond
//! `BASE`, and nothing it can acquire later.

mod base;
mod groups;

pub use base::BASE;
pub use groups::{BUILD_TOOLING, CRYPTO, DEBUG, IPC, NETWORK, SERVICE, STORAGE, WINDOW};
