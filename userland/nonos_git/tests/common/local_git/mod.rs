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
//! A transport backed by a real git serving a local repository.
//!
//! Smart HTTP is a thin shell around two commands: the advertisement comes
//! from `--advertise-refs` with the service banner prepended, and the request
//! body is handed to `--stateless-rpc` on stdin. Driving those directly means
//! the other end of this is genuine git, not a model of it.

mod banner;
mod transport;
mod types;

pub use types::LocalGit;
