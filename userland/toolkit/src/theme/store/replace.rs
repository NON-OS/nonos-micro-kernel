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
use core::sync::atomic::Ordering;

use super::state::{ACCENT, BG, BORDER, REVISION, SURFACE, TEXT};
use super::theme::Theme;

pub fn replace(new: Theme) {
    BG.store(new.background_argb, Ordering::Release);
    SURFACE.store(new.surface_argb, Ordering::Release);
    ACCENT.store(new.accent_argb, Ordering::Release);
    TEXT.store(new.text_argb, Ordering::Release);
    BORDER.store(new.border_argb, Ordering::Release);
    REVISION.fetch_add(1, Ordering::AcqRel);
}
