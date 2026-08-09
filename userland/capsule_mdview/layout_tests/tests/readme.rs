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

use mdview_layout_tests::layout::{parse, wrap, Line, Style};
use mdview_layout_tests::{line_width, measure, plain, words, README};

const CONTENT: i32 = 680 - 2 * 24;

fn readme_lines(limit: i32) -> Vec<Line> {
    let text = core::str::from_utf8(README).expect("seeded readme is utf-8");
    wrap(&parse(text), limit, measure)
}

#[test]
fn readme_parses_to_two_body_paragraphs() {
    let text = core::str::from_utf8(README).unwrap();
    let blocks = parse(text);
    assert_eq!(blocks.len(), 2);
    assert!(blocks.iter().all(|block| block.style == Style::Body));
}

#[test]
fn readme_wraps_and_marks_one_lead_per_block() {
    let lines = readme_lines(CONTENT);
    assert!(lines.len() > 2, "long paragraph must wrap: {lines:?}");
    assert_eq!(lines.iter().filter(|line| line.lead).count(), 2);
    assert!(lines[0].lead);
    assert_eq!(plain(&lines[0]), "Welcome to NONOS.");
}

#[test]
fn no_wrapped_line_overflows_the_content_width() {
    for line in readme_lines(CONTENT) {
        assert!(words(&line) > 1, "exemption would make this vacuous");
        assert!(
            line_width(&line) <= CONTENT || words(&line) == 1,
            "line overflows {CONTENT}px at {}px: {:?}",
            line_width(&line),
            plain(&line)
        );
    }
}

#[test]
fn wrapping_loses_no_text() {
    let joined: Vec<String> = readme_lines(CONTENT).iter().map(plain).collect();
    assert_eq!(
        joined.join(" "),
        "Welcome to NONOS. This file lives in the vfs capsule. Try: ls, cat \
         /docs/demo.txt, write /hello.txt hi, mkdir /tmp The file manager and \
         text editor see the same filesystem."
    );
}

#[test]
fn a_narrower_window_reflows_to_more_lines() {
    assert!(readme_lines(200).len() > readme_lines(CONTENT).len());
}
