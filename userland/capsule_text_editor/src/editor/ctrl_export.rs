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

use nonos_app_skeleton::{clients::vfs, EventOutcome};

use super::resolve_owner_pid::resolve_owner_pid;
use super::state::State;
use crate::doc::export::{docx, md, pdf};
use crate::doc::ttf_measure::TtfMeasurer;

pub(super) fn ctrl_export(state: &mut State, path_len: usize) -> EventOutcome {
    let path = state.prompt_path[..path_len].to_vec();
    let Some(bytes) = render(state, &path) else {
        state.status = b"export needs .md, .docx or .pdf";
        return EventOutcome::Repaint;
    };
    if !resolve_owner_pid(state) {
        state.status = b"export failed";
        return EventOutcome::Repaint;
    }
    let ok = vfs::write_file(state.owner_pid, &path, &bytes).is_ok();
    state.status = if ok { b"exported" } else { b"export failed" };
    EventOutcome::Repaint
}

fn render(state: &mut State, path: &[u8]) -> Option<Vec<u8>> {
    state.reflow();
    if ends_with(path, b".md") {
        return Some(md::to_markdown(&state.doc).into_bytes());
    }
    if ends_with(path, b".docx") {
        return Some(docx::to_docx(&state.doc));
    }
    if ends_with(path, b".pdf") {
        return Some(pdf::to_pdf(&state.doc, &state.page_metrics, &TtfMeasurer));
    }
    None
}

fn ends_with(path: &[u8], suffix: &[u8]) -> bool {
    path.len() > suffix.len() && path[path.len() - suffix.len()..].eq_ignore_ascii_case(suffix)
}
