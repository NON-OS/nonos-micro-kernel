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

// Literal bytes the vfs capsule seeds at /readme.txt, copied from
// userland/capsule_vfs/src/store/fdtable/seed.rs:22. mdview reads this exact
// file on startup, so the layout assertions run on the real payload.
pub const README: &[u8] = b"Welcome to NONOS.\n\nThis file lives in the vfs capsule.\nTry: ls, cat /docs/demo.txt, write /hello.txt hi, mkdir /tmp\nThe file manager and text editor see the same filesystem.\n";

// Constructs /readme.txt does not exercise: headings, an inline code span, a
// tight list, and a fenced code block.
pub const SAMPLE: &str = "# NONOS\n\nA `no_std` kernel.\n\n## Build\n\n- run make\n- boot it\n\n```\nmake nonos-mk\n```\n";

// A nested list and a fenced block inside a list item: both open a new block
// while an item block is already accumulating text.
pub const NESTED: &str = "- a\n  - b\n";

pub const NESTED_LOOSE: &str = "- a\n\n  - b\n\n  c\n";

pub const ITEM_CODE: &str = "- intro\n\n  ```\n  cmd\n  ```\n";
