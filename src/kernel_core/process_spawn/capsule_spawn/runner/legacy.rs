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

use super::super::spec::{CapsuleSpec, SpawnError};
use super::install::{install, InstallParams};

pub fn spawn(spec: &CapsuleSpec) -> Result<u32, SpawnError> {
    install(&InstallParams {
        name: spec.name,
        service_port: spec.service_port,
        reply_inbox: spec.reply_inbox,
        reply_port: spec.reply_port,
        elf: spec.elf,
        caps_bits: spec.caps_bits,
        debug_tag: spec.debug_tag,
        on_behalf_of: None,
    })
}
