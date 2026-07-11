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

use ab_glyph::FontRef;
use spin::Once;

// Body face (Noto Sans) for prose and UI with its true bold cut; monospace face
// (Noto Sans Mono) for pre/code. Noto has a tall x-height that stays legible at
// small sizes, and both cover Latin, Greek, Cyrillic, punctuation and common
// symbols; unmapped codepoints fall back to .notdef. Both are OFL, see
// assets/fonts/NotoSans-OFL.txt.
const BODY: &[u8] = include_bytes!("../../../assets/fonts/NotoSans-Regular.ttf");
const BOLD: &[u8] = include_bytes!("../../../assets/fonts/NotoSans-Bold.ttf");
const MONO: &[u8] = include_bytes!("../../../assets/fonts/NotoSansMono-Regular.ttf");

static BODY_FACE: Once<Option<FontRef<'static>>> = Once::new();
static BOLD_FACE: Once<Option<FontRef<'static>>> = Once::new();
static MONO_FACE: Once<Option<FontRef<'static>>> = Once::new();

// Parse once and hand back a shared borrow of the requested face. A corrupt
// asset yields None and every draw/measure degrades to a no-op rather than
// faulting.
pub(super) fn face(mono: bool) -> Option<&'static FontRef<'static>> {
    if mono {
        MONO_FACE.call_once(|| FontRef::try_from_slice(MONO).ok()).as_ref()
    } else {
        BODY_FACE.call_once(|| FontRef::try_from_slice(BODY).ok()).as_ref()
    }
}

// The built-in face for a weight and pitch. Bold monospace keeps the regular
// mono cut; callers thicken it themselves.
pub fn builtin_face(mono: bool, bold: bool) -> Option<&'static FontRef<'static>> {
    if !mono && bold {
        return BOLD_FACE.call_once(|| FontRef::try_from_slice(BOLD).ok()).as_ref();
    }
    face(mono)
}
