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

use crate::settings::schema::rows::{Live, Tone};
use crate::settings::state::State;

use super::build_info::{ARCHITECTURE, GIT_SHA, TOOLCHAIN, VERSION};
use super::live_net::{adapter, addr, link_state};
use super::valbuf::ValBuf;

/// Format one live row's value, and the tone it should read in.
pub fn resolve(state: &State, live: Live) -> (ValBuf, Tone) {
    let mut b = ValBuf::new();
    let tone = match live {
        Live::LinkState => return link_state(state),
        Live::IpAddress => addr(&mut b, state, |l| l.ip),
        Live::Gateway => addr(&mut b, state, |l| l.gw),
        Live::Dns => addr(&mut b, state, |l| l.dns),
        Live::Adapter => adapter(&mut b, state),
        Live::Version => text(&mut b, VERSION.trim()),
        Live::Commit => text(&mut b, GIT_SHA),
        Live::Toolchain => text(&mut b, TOOLCHAIN),
        Live::Architecture => text(&mut b, ARCHITECTURE),
        Live::StorageService => text(&mut b, "Not exported"),
    };
    (b, tone)
}

fn text(b: &mut ValBuf, s: &str) -> Tone {
    b.push_str(s);
    Tone::Idle
}
