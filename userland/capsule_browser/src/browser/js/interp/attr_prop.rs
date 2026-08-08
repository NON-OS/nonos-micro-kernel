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

/// The attribute a property reads and writes, when it is one.
///
/// Most of what a script sets on an element is not stored on the element at
/// all: `el.href = "/a"` and `el.setAttribute("href", "/a")` are the same
/// write. Answering undefined for the property form means a link built by a
/// script has no destination and an image no source, even though the code
/// that made it is correct.
///
/// The names that differ are the ones a script cannot guess: the attribute
/// is spelled the way markup spells it, the property the way JavaScript
/// does, and `for` and `class` had to be renamed because they are keywords.
pub fn attr_prop(name: &str) -> Option<&'static str> {
    Some(match name {
        "href" => "href",
        "src" => "src",
        "alt" => "alt",
        "title" => "title",
        "type" => "type",
        "name" => "name",
        "rel" => "rel",
        "target" => "target",
        "placeholder" => "placeholder",
        "action" => "action",
        "method" => "method",
        "width" => "width",
        "height" => "height",
        "role" => "role",
        "lang" => "lang",
        "dir" => "dir",
        "htmlFor" => "for",
        "tabIndex" => "tabindex",
        "colSpan" => "colspan",
        "rowSpan" => "rowspan",
        "maxLength" => "maxlength",
        "srcset" => "srcset",
        "sizes" => "sizes",
        "loading" => "loading",
        "content" => "content",
        _ => return None,
    })
}

/// Properties that are present or absent rather than set to a string.
///
/// `el.disabled = true` writes the attribute, `= false` removes it, and
/// reading gives a boolean. Writing the string "false" would leave the
/// element disabled, which is the bug this exists to avoid.
pub fn bool_prop(name: &str) -> Option<&'static str> {
    Some(match name {
        "disabled" => "disabled",
        "checked" => "checked",
        "readOnly" => "readonly",
        "required" => "required",
        "multiple" => "multiple",
        "selected" => "selected",
        "open" => "open",
        "hidden" => "hidden",
        _ => return None,
    })
}
