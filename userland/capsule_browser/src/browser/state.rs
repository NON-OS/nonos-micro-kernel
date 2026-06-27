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

use alloc::string::String;
use alloc::vec::Vec;

pub enum View {
    Home,
    Page,
}

pub struct State {
    pub address: String,
    pub address_focused: bool,
    pub status: String,
    pub pending_nav: Option<String>,
    pub document: Option<crate::browser::layout::doc::RenderDocument>,
    pub scroll: u32,
    pub dns_port: u32,
    pub sockets_port: u32,
    pub view: View,
    pub fetch: Option<crate::browser::fetch::types::Fetch>,
    pub base: Option<crate::browser::url::Url>,
    pub redirect_count: u8,
    pub history: Vec<String>,
    pub hist_index: i32,
    pub suppress_history_push: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            address: String::new(),
            address_focused: true,
            status: String::from("ready"),
            pending_nav: None,
            document: None,
            scroll: 0,
            dns_port: 0,
            sockets_port: 0,
            view: View::Home,
            fetch: None,
            base: None,
            redirect_count: 0,
            history: Vec::new(),
            hist_index: -1,
            suppress_history_push: false,
        }
    }
}
