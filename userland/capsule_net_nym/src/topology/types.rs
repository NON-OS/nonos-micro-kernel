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

pub const DIR_MAGIC: [u8; 4] = *b"NYMD";
pub const DIR_VERSION: u8 = 1;
pub const DIR_HEADER_LEN: usize = 128;
pub const NODE_CAP: usize = 128;
/// A node record on the wire. Carries both ports a node answers on: the mix
/// port a packet is routed to, and the websocket port a client dials.
pub const NODE_WIRE_LEN: usize = 76;
/// Hops a header holds a layer for: one per mix layer, then the gateway the
/// packet leaves by. Five nodes carry a packet, but our own entry gateway is
/// handed it directly and only forwards it, so it is not one of these.
pub const ROUTE_HOPS: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Role {
    EntryGateway,
    Mix,
    ExitGateway,
}

#[derive(Clone, Copy)]
pub struct Node {
    pub role: Role,
    pub layer: u8,
    pub delay_ms: u16,
    pub ip: [u8; 4],
    /// Where a packet is routed to. This is the address a header names, so
    /// it is the mix port even for a gateway.
    pub port: u16,
    /// Where a client dials to hold a session. Gateways answer on a
    /// different port from the one they take packets on, so a route address
    /// cannot double as one.
    pub ws_port: u16,
    pub identity: [u8; 32],
    pub packet_key: [u8; 32],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TopologyError {
    BadLength,
    BadMagic,
    BadVersion,
    BadTime,
    BadSignature,
    Clock,
    Empty,
    NoAuthority,
    Stale,
    TooLarge,
    UntrustedAuthority,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RouteError {
    Empty,
    Expired,
    MissingHop,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TopologyStatus {
    Missing,
    Ready,
    Expired,
    Clock,
    UntrustedAuthority,
}
