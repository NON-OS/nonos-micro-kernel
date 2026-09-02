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

use alloc::vec::Vec;

use crate::jobs::JobTable;
use crate::term::block::Block;
use crate::term::cwd::Cwd;
use crate::term::history::History;
use crate::term::line::Line;
use crate::term::scrollback::Scrollback;

pub struct State {
    pub line: Line,
    pub history: History,
    pub scrollback: Scrollback,
    pub cwd: Cwd,
    pub owner_pid: u32,
    pub fresh: bool,
    pub start_ms: u64,
    // Shell variables, set with `set NAME VALUE` and expanded as $NAME.
    pub vars: Vec<(Vec<u8>, Vec<u8>)>,
    // Exit status of the last command: 0 on success, nonzero on failure.
    // Fallible commands set it on error; `&&` and `||` gate the next
    // statement on it.
    pub last_status: i32,
    // Command aliases, defined with `alias NAME EXPANSION`. The first word
    // of a line is replaced by its expansion before the line is run.
    pub aliases: Vec<(Vec<u8>, Vec<u8>)>,
    // The text typed when history navigation began. Up and Down recall only
    // entries that start with it, and restore it when the search runs off
    // the newest entry.
    pub hist_prefix: Vec<u8>,
    /// An in-progress reverse search, or nothing when not searching.
    pub search: Option<crate::term::search::Search>,
    pub blocks: Vec<Block>,
    // Theme and zoom belong to the window, not the tab, so the deep command
    // and key paths post a request here and Terminal drains it into the state
    // it owns. None / 0 mean nothing was asked for.
    pub theme_req: Option<u16>,
    pub zoom_req: i32,
    pub jobs: JobTable,
    // Set when a statement is submitted as a foreground job: on_enter
    // stops consuming the line (v1 limit: statements after a foreground
    // job are dropped, so it should be the last on its line) and defers
    // close_block and last_status to the job's reap in the on_tick pump.
    pub fg_running: bool,
    pub fg_started_ms: i64,
}
