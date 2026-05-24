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

use crate::protocol::{
    E_AUTHORITY_MISSING, E_AUTHORITY_UNTRUSTED, E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION, E_CRYPTO,
    E_TOPOLOGY_AUTH, E_TOPOLOGY_STALE,
};
use crate::topology;

pub fn map(e: topology::TopologyError) -> u16 {
    match e {
        topology::TopologyError::BadMagic => E_BAD_MAGIC,
        topology::TopologyError::BadVersion => E_BAD_VERSION,
        topology::TopologyError::BadSignature => E_TOPOLOGY_AUTH,
        topology::TopologyError::NoAuthority => E_AUTHORITY_MISSING,
        topology::TopologyError::UntrustedAuthority => E_AUTHORITY_UNTRUSTED,
        topology::TopologyError::BadTime | topology::TopologyError::Stale => E_TOPOLOGY_STALE,
        topology::TopologyError::Clock => E_CRYPTO,
        topology::TopologyError::BadLength
        | topology::TopologyError::Empty
        | topology::TopologyError::TooLarge => E_BAD_LEN,
    }
}
