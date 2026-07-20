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

use smoltcp::socket::dhcpv4;

use crate::iface::dhcp::types::{ConfiguredLease, DhcpAction};
use crate::iface::dhcp::{handle_configured, handle_deconfigured};
use crate::state;

pub fn poll_event() {
    state::with_dhcp_and_dns_slot(|iface, sockets, dhcp_handle, dns_slot| {
        let action = match sockets.get_mut::<dhcpv4::Socket>(dhcp_handle).poll() {
            Some(dhcpv4::Event::Configured(cfg)) => DhcpAction::Configured(ConfiguredLease {
                address: cfg.address,
                router: cfg.router,
                dns: match cfg.dns_servers.first() {
                    Some(d) => d.0,
                    None => [0u8; 4],
                },
            }),
            Some(dhcpv4::Event::Deconfigured) => DhcpAction::Deconfigured,
            None => DhcpAction::None,
        };
        match action {
            DhcpAction::Configured(cfg) => {
                handle_configured::handle_configured(iface, sockets, dns_slot, cfg);
            }
            DhcpAction::Deconfigured => {
                handle_deconfigured::handle_deconfigured(iface, sockets, dns_slot);
            }
            DhcpAction::None => {}
        }
    });
}
