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

use alloc::string::String;

use super::apply::apply_decl;
use super::computed::Computed;
use super::parse::parse_decls;

pub fn apply_style_attr(
    style: &str,
    c: &mut Computed,
    parent_fs: u32,
    vars: &[(String, String)],
    bg: &mut Option<String>,
) {
    for d in parse_decls(style) {
        apply_decl(c, &d.name, &d.value, parent_fs, vars);
        if let Some(u) = super::bg_url::bg_url(&d.name, &d.value) {
            *bg = Some(u);
        }
    }
}
