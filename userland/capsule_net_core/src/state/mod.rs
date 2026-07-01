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

mod globals;
mod lease;
mod store;
mod types;
mod with_dhcp_and_dns_slot;
mod with_dns;
mod with_iface;

pub use lease::{lease, set_lease};
pub use store::store;
pub use types::{Lease, NetState};
pub use with_dhcp_and_dns_slot::with_dhcp_and_dns_slot;
pub use with_dns::with_dns;
pub use with_iface::with_iface;
