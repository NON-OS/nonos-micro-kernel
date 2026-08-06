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

use crate::wire::Host;

/// The largest domain name a destination carries.
pub const DOMAIN_MAX: usize = 255;

/// A destination for the tunnel, copied out of the transient request buffer so
/// it outlives the handshake bytes.
///
/// The domain variant carries its name inline rather than behind a pointer.
/// Boxing it would shrink the enum and put an allocation on the path every
/// connection takes, which is the wrong trade in a capsule that keeps a fixed
/// table and no allocator pressure.
#[allow(clippy::large_enum_variant)]
pub enum Dest {
    V4([u8; 4], u16),
    V6([u8; 16], u16),
    Domain { name: [u8; DOMAIN_MAX], len: u8, port: u16 },
}

impl Dest {
    pub(super) fn from_host(host: &Host<'_>, port: u16) -> Self {
        match host {
            Host::V4(a) => Dest::V4(*a, port),
            Host::V6(a) => Dest::V6(*a, port),
            Host::Domain(d) => {
                let mut name = [0u8; DOMAIN_MAX];
                let len = d.len().min(DOMAIN_MAX);
                name[..len].copy_from_slice(&d[..len]);
                Dest::Domain { name, len: len as u8, port }
            }
        }
    }
}
