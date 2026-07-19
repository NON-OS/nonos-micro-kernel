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

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::grid_spec::GridSpec;

// Capture the named-grid declarations into the per-node side spec. Runs
// beside the normal property apply, so cascade order (later wins) holds.
pub(super) fn grid_decl(spec: &mut Option<GridSpec>, name: &str, value: &str, em: u32) {
    match name {
        "grid-template-columns" => {
            let lines = super::grid_lines::col_line_names(value, em);
            if !lines.is_empty() {
                ensure(spec).col_lines = lines;
            }
        }
        "grid-template-areas" => {
            let rows = area_rows(value);
            if !rows.is_empty() {
                ensure(spec).areas = rows;
            }
        }
        "grid-area" => {
            let parts: Vec<&str> = value.split('/').map(str::trim).collect();
            match parts.as_slice() {
                [one] => {
                    if !one.is_empty() && one.parse::<i16>().is_err() {
                        ensure(spec).area = Some(one.to_string());
                    }
                }
                // row-start / col-start [/ row-end / col-end]
                [r, c] => {
                    let s = ensure(spec);
                    s.row_start = r.parse::<i16>().ok();
                    s.col_start = Some((*c).to_string());
                }
                [r0, c0, r1, c1] => {
                    let s = ensure(spec);
                    s.row_start = r0.parse::<i16>().ok();
                    s.col_start = Some((*c0).to_string());
                    s.row_end = r1.parse::<i16>().ok();
                    s.col_end = Some((*c1).to_string());
                }
                _ => {}
            }
        }
        "grid-column" => {
            let mut it = value.split('/').map(str::trim);
            if let Some(a) = it.next() {
                if !a.is_empty() {
                    let s = ensure(spec);
                    s.col_start = Some(a.to_string());
                    s.col_end = it.next().filter(|b| !b.is_empty()).map(|b| b.to_string());
                }
            }
        }
        "grid-column-start" => ensure(spec).col_start = Some(value.trim().to_string()),
        "grid-column-end" => ensure(spec).col_end = Some(value.trim().to_string()),
        "grid-row" => {
            let mut it = value.split('/').map(str::trim);
            if let Some(a) = it.next() {
                if let Ok(r) = a.parse::<i16>() {
                    let s = ensure(spec);
                    s.row_start = Some(r);
                    s.row_end = it.next().and_then(|b| b.parse::<i16>().ok());
                }
            }
        }
        "grid-row-start" => {
            if let Ok(r) = value.trim().parse::<i16>() {
                ensure(spec).row_start = Some(r);
            }
        }
        "grid-row-end" => {
            if let Ok(r) = value.trim().parse::<i16>() {
                ensure(spec).row_end = Some(r);
            }
        }
        _ => {}
    }
}

fn ensure(spec: &mut Option<GridSpec>) -> &mut GridSpec {
    spec.get_or_insert_with(GridSpec::default)
}

// The quoted rows of a grid-template-areas value, each split into cell
// tokens. Rows are capped so a hostile sheet cannot balloon the table.
fn area_rows(value: &str) -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut rest = value;
    while rows.len() < 16 {
        let Some(open) = rest.find('"') else { break };
        let Some(len) = rest[open + 1..].find('"') else { break };
        let row = &rest[open + 1..open + 1 + len];
        let cells: Vec<String> = row.split_whitespace().take(32).map(|c| c.to_string()).collect();
        if !cells.is_empty() {
            rows.push(cells);
        }
        rest = &rest[open + len + 2..];
    }
    rows
}
