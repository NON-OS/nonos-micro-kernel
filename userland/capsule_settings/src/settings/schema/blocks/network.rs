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

use nonos_policy_proto::Field;

use crate::settings::schema::rows::{Block, Live, Pill, Row};

pub const NETWORK: &[Block] = &[
    Block {
        title: "Network status",
        note: Some("Where this device stands with the DHCP client."),
        pill: Pill::Net,
        rows: &[
            Row::Live("Connection", Live::LinkState),
            Row::Live("IP address", Live::IpAddress),
            Row::Live("Gateway", Live::Gateway),
            Row::Live("DNS", Live::Dns),
        ],
    },
    Block {
        title: "Network options",
        note: None,
        pill: Pill::None,
        rows: &[
            Row::Field(Field::WifiAutoconnect),
            Row::Field(Field::PreferIpv6),
            Row::Field(Field::MeteredConnection),
            Row::Field(Field::ProxyMode),
        ],
    },
    Block {
        title: "Interfaces",
        note: None,
        pill: Pill::None,
        rows: &[Row::Live("Adapter", Live::Adapter)],
    },
];
