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
use nonos_app_skeleton::clients::vfs::list_paths;
use nonos_app_skeleton::EventOutcome;

use super::complete::{command_candidates, common_prefix};
use crate::term::cwd::resolve;
use crate::term::state::State;

// Tab completion. The first token completes against command names; any
// later token completes against VFS paths. The completion extends the
// word by the candidates' common prefix; if nothing can be extended and
// several candidates exist, they are listed.
pub fn on_tab(state: &mut State) -> EventOutcome {
    let line = state.line.as_bytes().to_vec();
    let start = line.iter().rposition(|&b| b == b' ').map(|i| i + 1).unwrap_or(0);
    let word = &line[start..];
    let (cands, base): (Vec<Vec<u8>>, Vec<u8>) = if start == 0 {
        (command_candidates(word).iter().map(|c| c.to_vec()).collect(), word.to_vec())
    } else {
        let resolved = resolve(state.cwd.as_bytes(), word);
        match list_paths(state.owner_pid, &resolved) {
            Ok(paths) => (paths.iter().map(|s| s.as_bytes().to_vec()).collect(), resolved),
            Err(_) => return EventOutcome::Repaint,
        }
    };
    if cands.is_empty() {
        return EventOutcome::Repaint;
    }
    let refs: Vec<&[u8]> = cands.iter().map(|c| c.as_slice()).collect();
    let cp = common_prefix(&refs);
    let extension = cp.get(base.len()..).unwrap_or(b"");
    if !extension.is_empty() {
        let mut nl = line[..start].to_vec();
        nl.extend_from_slice(word);
        nl.extend_from_slice(extension);
        state.line.replace(&nl);
    } else if cands.len() > 1 {
        let mut row = Vec::new();
        for c in &cands {
            if !row.is_empty() {
                row.push(b' ');
            }
            row.extend_from_slice(c);
        }
        state.scrollback.push_line(&row);
    }
    EventOutcome::Repaint
}
