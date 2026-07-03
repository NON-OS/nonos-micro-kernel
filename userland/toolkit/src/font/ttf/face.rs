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

// Body face (Inter) for prose and UI; monospace face (JetBrains Mono, OFL) for
// pre/code. Both cover Latin, punctuation and common symbols; unmapped
// codepoints fall back to .notdef.
const BODY: &[u8] = include_bytes!("../../../assets/fonts/Inter-Regular.ttf");
const MONO: &[u8] = include_bytes!("../../../assets/fonts/JetBrainsMono-Regular.ttf");

static BODY_FACE: Once<Option<FontRef<'static>>> = Once::new();
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
