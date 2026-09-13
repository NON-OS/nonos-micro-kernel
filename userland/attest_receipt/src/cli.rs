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

//! How to ask.
//!
//! ```text
//! nonos-receipt <entries> <signed root hex>
//! ```
//!
//! The root is an argument rather than something read out of a document: a
//! reader that dug it out itself would invite trusting a root nobody checked a
//! signature on. Verifying the TPM quote is the caller's step, before this one.

use std::path::PathBuf;

use crate::hexcode::parse_root;

pub enum Mode {
    Root { entries: PathBuf, root: [u8; 32] },
}

pub const USAGE: &str = "usage: nonos-receipt <entries> <signed root, hex>";

pub fn parse_args(args: &[String]) -> Result<Mode, String> {
    match args {
        [entries, root] => {
            let root = parse_root(root).ok_or("the root must be 64 hex characters")?;
            Ok(Mode::Root { entries: entries.into(), root })
        }
        _ => Err(USAGE.to_string()),
    }
}
