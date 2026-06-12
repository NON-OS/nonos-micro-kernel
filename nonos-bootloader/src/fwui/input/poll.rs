// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::nav::Nav;
use uefi::prelude::*;
use uefi::proto::console::text::{Input, Key, ScanCode};

pub fn poll(bs: &BootServices) -> Nav {
    let Ok(handle) = bs.get_handle_for_protocol::<Input>() else {
        return Nav::None;
    };
    let Ok(mut input) = bs.open_protocol_exclusive::<Input>(handle) else {
        return Nav::None;
    };
    match input.read_key() {
        Ok(Some(Key::Special(sc))) => match sc {
            ScanCode::UP => Nav::Up,
            ScanCode::DOWN => Nav::Down,
            ScanCode::LEFT => Nav::Left,
            ScanCode::RIGHT => Nav::Right,
            ScanCode::ESCAPE => Nav::Back,
            ScanCode::FUNCTION_10 => Nav::BootNow,
            _ => Nav::None,
        },
        Ok(Some(Key::Printable(ch))) => match char::from_u32(u16::from(ch) as u32) {
            Some('\r') | Some('\n') => Nav::Enter,
            Some('w') | Some('W') | Some('k') | Some('K') => Nav::Up,
            Some('s') | Some('S') | Some('j') | Some('J') => Nav::Down,
            Some('a') | Some('A') | Some('h') | Some('H') => Nav::Left,
            Some('d') | Some('D') | Some('l') | Some('L') => Nav::Right,
            Some('q') | Some('Q') => Nav::Back,
            _ => Nav::None,
        },
        _ => Nav::None,
    }
}
