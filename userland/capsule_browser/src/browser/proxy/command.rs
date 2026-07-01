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

use crate::browser::proxy::parse_socks5;
use crate::browser::state::State;

pub fn command(state: &mut State, input: &str) -> bool {
    let Some(rest) = input.trim().strip_prefix("proxy ") else {
        return false;
    };
    if rest.trim() == "off" {
        state.proxy = None;
        state.status = alloc::string::String::from("proxy off");
        return true;
    }
    match parse_socks5::parse_socks5(rest.trim()) {
        Some(cfg) => {
            state.status = alloc::format!("proxy socks5://{}:{}", cfg.host, cfg.port);
            state.proxy = Some(cfg);
        }
        None => state.status = alloc::string::String::from("bad proxy"),
    }
    true
}
