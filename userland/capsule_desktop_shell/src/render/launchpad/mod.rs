// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The Launchpad: a full-screen grid of every desktop app and installed tool,
//! opened from the dock. Drawing and hit-testing live here; the click action
//! that launches a tile and dismisses the overlay lives in the server layer.

mod dots;
mod gen_icon;
mod grid;
mod hit;
mod paint;
mod search;
mod tile;
mod tool_icons;
mod view;

pub use hit::{hit_target, Target};
pub use paint::paint_launchpad;
pub(crate) use dots::hit as dots_hit;
pub(crate) use search::hit as search_hit;
pub(crate) use view::{page_slice, rebuild};
