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

mod authority;
mod default_port;
mod dot_segments;
mod has_scheme_prefix;
mod host_port;
mod join;
mod normalize_path;
mod parse;
mod path_without_fragment;
mod path_without_query;
mod request_target;
mod scheme_rest;
mod split_path;
mod types;

pub use authority::authority;
pub use join::join;
pub use parse::parse;
pub use request_target::request_target;
pub use types::{Scheme, Url};
