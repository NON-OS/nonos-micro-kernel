// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Installed command-line tools shown in the Launchpad. Empty until the
//! nonos-app pipeline installs some; the Launchpad shows the desktop apps.

/// One installed tool: the name on its tile and the service it launches under.
pub struct ToolApp {
    pub label: &'static [u8],
    pub service: &'static [u8],
}

/// Every installed tool. None yet, so the Launchpad grid is the desktop apps.
pub const TOOL_APPS: [ToolApp; 0] = [];
