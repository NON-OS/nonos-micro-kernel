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

use nonos_libc::mk_getpid;

use super::types::State;
use crate::jobs::JobTable;
use crate::term::cwd::{Cwd, HOME};
use crate::term::history::History;
use crate::term::line::Line;
use crate::term::scrollback::Scrollback;

impl State {
    pub fn new() -> Self {
        Self {
            line: Line::new(),
            history: History::new(),
            scrollback: Scrollback::new(),
            cwd: Cwd::new(),
            owner_pid: mk_getpid(),
            fresh: true,
            start_ms: 0,
            vars: alloc::vec![(alloc::vec::Vec::from(&b"HOME"[..]), alloc::vec::Vec::from(HOME))],
            last_status: 0,
            aliases: alloc::vec::Vec::new(),
            hist_prefix: alloc::vec::Vec::new(),
            search: None,
            blocks: alloc::vec::Vec::new(),
            theme_req: None,
            zoom_req: 0,
            jobs: JobTable::new(),
            fg_running: false,
            fg_started_ms: 0,
        }
    }
}
