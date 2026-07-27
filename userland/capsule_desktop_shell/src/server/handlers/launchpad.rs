// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Opening the Launchpad from the dock, and acting on a click while it is open:
//! a tile launches its app or tool, and any click then dismisses the overlay.

use crate::render::launchpad::{hit, target, Target};
use crate::server::handlers::launcher_request;
use crate::server::repaint::repaint;
use crate::state::{Context, LAUNCHER_APPS, TOOL_APPS};

pub fn open(ctx: &mut Context) {
    ctx.launchpad = true;
    repaint(ctx);
}

pub fn click(ctx: &mut Context, px: u32, py: u32) {
    if let Some(index) = hit(ctx.width, px, py) {
        match target(index) {
            Target::App(a) => {
                let _ = launcher_request::request(&LAUNCHER_APPS[a]);
            }
            Target::Tool(t) => {
                let _ = launcher_request::request_service(TOOL_APPS[t].service);
            }
        }
    }
    // A click anywhere closes the Launchpad, whether it landed on a tile or on
    // empty space, matching how a full-screen launcher is expected to behave.
    ctx.launchpad = false;
    repaint(ctx);
}
