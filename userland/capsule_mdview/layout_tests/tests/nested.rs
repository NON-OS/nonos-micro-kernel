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

use mdview_layout_tests::layout::{parse, Block, Style};
use mdview_layout_tests::{ITEM_CODE, NESTED, NESTED_LOOSE};

fn text(block: &Block) -> String {
    block.spans.iter().map(|span| span.text.as_str()).collect()
}

fn shape(markdown: &str) -> Vec<(Style, String)> {
    parse(markdown)
        .iter()
        .map(|block| (block.style, text(block).trim().to_string()))
        .collect()
}

#[test]
fn a_nested_list_does_not_swallow_the_outer_item() {
    assert_eq!(
        shape(NESTED),
        vec![
            (Style::Bullet, String::from("a")),
            (Style::Bullet, String::from("b")),
        ]
    );
}

#[test]
fn a_code_block_inside_an_item_does_not_swallow_the_item() {
    assert_eq!(
        shape(ITEM_CODE),
        vec![
            (Style::Bullet, String::from("intro")),
            (Style::Code, String::from("cmd")),
        ]
    );
}

#[test]
fn an_outer_item_stays_a_bullet_after_its_nested_list_closes() {
    assert_eq!(
        shape(NESTED_LOOSE),
        vec![
            (Style::Bullet, String::from("a")),
            (Style::Bullet, String::from("b")),
            (Style::Bullet, String::from("c")),
        ]
    );
}
