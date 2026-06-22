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

use uefi::prelude::*;
use uefi::proto::console::text::{Input, Key, ScanCode};

use super::nav::Nav;

pub(super) fn poll(bs: &BootServices) -> Nav {
    let Ok(handle) = bs.get_handle_for_protocol::<Input>() else {
        return Nav::None;
    };
    let Ok(mut input) = bs.open_protocol_exclusive::<Input>(handle) else {
        return Nav::None;
    };
    match input.read_key() {
        Ok(Some(Key::Special(ScanCode::UP))) => Nav::Up,
        Ok(Some(Key::Special(ScanCode::DOWN))) => Nav::Down,
        Ok(Some(Key::Printable(ch))) => match char::from_u32(u16::from(ch) as u32) {
            Some('\r') | Some('\n') => Nav::Enter,
            Some('w') | Some('W') | Some('k') | Some('K') => Nav::Up,
            Some('s') | Some('S') | Some('j') | Some('J') => Nav::Down,
            _ => Nav::None,
        },
        _ => Nav::None,
    }
}
