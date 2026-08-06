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

use super::env::Env;
use super::value::Value;

// A queued callback: fires when `left` ticks reach zero; `every` reloads it
// for setInterval.
pub struct Timer {
    pub cb: Value,
    pub left: u32,
    pub every: Option<u32>,
}

// The page's live script state between events: the global scope, the event
// listeners keyed by node id, pending timers, and script-issued requests
// (queued url + callback, plus the callback of the request on the wire).
pub struct World {
    pub env: Env,
    pub listeners: Vec<(usize, String, Value)>,
    pub timers: Vec<Timer>,
    pub net: Vec<(String, Value)>,
    pub net_active: Option<Value>,
}

impl World {
    // An empty script world: a root scope and no listeners, timers or pending
    // requests. Page scripts run through QuickJS now, so this is the inert
    // companion the timer and script-fetch pumps read while the engine holds
    // the live page state.
    pub fn empty() -> World {
        let env = Env::root();
        super::interp::install(&env);
        World { env, listeners: Vec::new(), timers: Vec::new(), net: Vec::new(), net_active: None }
    }
}
