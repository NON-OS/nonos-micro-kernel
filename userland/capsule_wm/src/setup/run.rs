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

use nonos_libc::mk_yield;

use super::discover;
use crate::compositor_client::{probe_compositor, query_display_info};
use crate::focus::FocusModel;
use crate::state::{Context, SubscriptionList};
use crate::window::WindowTable;
use crate::z_order::ZStack;

const READY_ATTEMPTS: usize = 256;

pub fn run() -> Result<Context, &'static str> {
    let mut last_err = "compositor unavailable";
    for _ in 0..READY_ATTEMPTS {
        match run_once() {
            Ok(ctx) => return Ok(ctx),
            Err(e) => {
                last_err = e;
                mk_yield();
            }
        }
    }
    Err(last_err)
}

fn run_once() -> Result<Context, &'static str> {
    let compositor_port = discover::lookup_compositor_port()?;
    probe_compositor(compositor_port, 1)?;
    let display = query_display_info(compositor_port, 2)?;
    Ok(Context {
        compositor_port,
        display_width: display.width,
        display_height: display.height,
        windows: WindowTable::new(),
        focus: FocusModel::new(),
        z: ZStack::new(),
        subscriptions: SubscriptionList::new(),
        next_request_id: 3,
    })
}
