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

//! A store name is unsigned text: no manifest field carries a signed display
//! name, so nothing on the wire stops an installed capsule from calling itself
//! Terminal, Wallet or Settings. A name is admitted only when it keeps the
//! store's own character contract and claims neither the label nor the
//! service of a built-in entry, in either the launcher or the tool table.

use crate::state::{LAUNCHER_APPS, TOOL_APPS};

use super::constants::MAX_NAME;

const APP_PREFIX: &[u8] = b"app.";
const TOOL_PREFIX: &[u8] = b"tool.";

pub(super) fn admissible(name: &[u8]) -> bool {
    well_formed(name) && !impersonates_builtin(name)
}

fn well_formed(name: &[u8]) -> bool {
    !name.is_empty()
        && name.len() <= MAX_NAME
        && name.iter().all(|b| b.is_ascii_alphanumeric() || *b == b'_' || *b == b'-')
}

fn impersonates_builtin(name: &[u8]) -> bool {
    LAUNCHER_APPS
        .iter()
        .any(|app| claims(app.label, app.service, APP_PREFIX, name))
        || TOOL_APPS
            .iter()
            .any(|app| claims(app.label, app.service, TOOL_PREFIX, name))
}

fn claims(label: &[u8], service: &[u8], prefix: &[u8], name: &[u8]) -> bool {
    label.eq_ignore_ascii_case(name)
        || service
            .strip_prefix(prefix)
            .is_some_and(|base| base.eq_ignore_ascii_case(name))
}
