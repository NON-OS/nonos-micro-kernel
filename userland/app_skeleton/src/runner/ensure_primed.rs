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

use crate::app::App;
use crate::discover::Peers;

use super::boot::BootedApp;
use super::paint_once::paint_once;

pub(super) fn ensure_primed<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
) -> bool {
    if booted.primed {
        return true;
    }
    booted.primed = paint_once(
        &mut booted.app,
        &booted.manifest,
        &booted.binding,
        peers,
        request_id,
        booted.maximized,
    );
    booted.primed
}
