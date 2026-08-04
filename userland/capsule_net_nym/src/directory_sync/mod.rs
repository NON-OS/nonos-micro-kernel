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

mod api;
mod http;
mod https;
mod live;
mod resolve;
mod tls_io;
mod source;

pub use api::{objects, parse_node};
pub use http::fetch;
pub use https::fetch_tls;
pub use live::sync as sync_live;
pub use resolve::resolve;
pub use tls_io::TcpIo;
pub use source::{parse, DirectorySource};
