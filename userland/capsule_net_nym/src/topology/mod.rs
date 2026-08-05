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

pub(crate) mod admissible;
mod builtin;
mod by_identity;
mod clock;
mod directory;
mod fetched;
mod layout;
mod node;
mod parse;
mod select;
mod status;
mod store;
mod types;
mod verify;

pub use builtin::install as install_builtin;
pub use by_identity::node_by_identity;
pub use directory::DirectoryMeta;
pub use fetched::install_fetched;
pub use parse::install;
pub use select::route;
pub use status::current as status;
pub use store::{meta, snapshot};
pub use types::{Node, Role, TopologyError, TopologyStatus, NODE_CAP, ROUTE_HOPS};
