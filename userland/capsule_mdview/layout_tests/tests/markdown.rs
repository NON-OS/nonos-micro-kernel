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

use mdview_layout_tests::layout::{parse, wrap, Style};
use mdview_layout_tests::{measure, plain, SAMPLE};

const CONTENT: i32 = 680 - 2 * 24;

#[test]
fn sample_block_styles_follow_source_order() {
    let styles: Vec<Style> = parse(SAMPLE).iter().map(|block| block.style).collect();
    assert_eq!(
        styles,
        vec![
            Style::H1,
            Style::Body,
            Style::H2,
            Style::Bullet,
            Style::Bullet,
            Style::Code
        ]
    );
}

#[test]
fn heading_and_code_block_keep_their_text() {
    let lines = wrap(&parse(SAMPLE), CONTENT, measure);
    assert_eq!(plain(&lines[0]), "NONOS");
    let code = lines
        .iter()
        .find(|line| line.style == Style::Code)
        .expect("fenced block becomes a code line");
    assert_eq!(plain(code), "make nonos-mk");
    assert!(code.spans[0].mono);
}

#[test]
fn inline_code_survives_wrapping_as_its_own_mono_run() {
    let lines = wrap(&parse(SAMPLE), CONTENT, measure);
    let body = &lines[1];
    assert_eq!(plain(body), "A no_std kernel.");
    assert!(
        body.spans
            .iter()
            .any(|span| span.mono && span.text.trim() == "no_std"),
        "inline code lost its mono run: {:?}",
        body.spans
    );
}

#[test]
fn every_list_item_opens_its_own_block() {
    let lines = wrap(&parse(SAMPLE), CONTENT, measure);
    let bullets: Vec<String> = lines
        .iter()
        .filter(|line| line.style == Style::Bullet)
        .map(plain)
        .collect();
    assert_eq!(bullets, vec!["run make", "boot it"]);
}
