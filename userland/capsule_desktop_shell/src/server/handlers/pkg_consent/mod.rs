// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The consent gate for installing a `.nonos` package: Approve commits it via
//! the installer, any other click dismisses. Geometry lives beside it as the
//! single source of truth the paint pass hit-tests against.

mod click;
mod geometry;

pub(crate) use click::click;
pub(crate) use geometry::{approve_rect, cancel_rect, panel_rect};
