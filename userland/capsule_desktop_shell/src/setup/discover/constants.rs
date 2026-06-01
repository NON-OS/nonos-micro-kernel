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

use core::sync::atomic::AtomicBool;

pub(super) const COMPOSITOR_SERVICE: &[u8] = b"compositor";
pub(super) const MARKET_SERVICE: &[u8] = b"market.index";
pub(super) const MARKET_UNAVAILABLE_LEN: usize = 75;
pub(super) const MARKET_UNAVAILABLE: &[u8] =
    b"[DESKTOP] optional service market.index unavailable; disabling market panel";
pub(super) const WALLPAPER_SERVICE: &[u8] = b"wallpaper";
pub(super) const WM_SERVICE: &[u8] = b"wm";
pub(super) static MARKET_DISABLED: AtomicBool = AtomicBool::new(false);
