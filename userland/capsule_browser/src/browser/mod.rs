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

extern crate alloc;

mod app;
mod css;
pub mod dom;
mod event;
pub mod fetch;
pub mod fonts;
pub mod html;
pub mod http;
pub mod image;
mod js;
mod keymap;
pub mod layout;
pub mod manifest;
mod net;
mod paint;
mod proxy;
pub mod state;
pub mod tls13;
pub mod url;

pub use app::Browser;
