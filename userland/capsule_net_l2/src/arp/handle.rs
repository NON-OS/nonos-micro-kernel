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

use super::cache::Cache;
use super::packet::{ArpPacket, OPER_REPLY, OPER_REQUEST, PACKET_LEN};
use crate::ethernet::{EthHeader, ETHERTYPE_ARP, HDR_LEN, MAC_BROADCAST};

pub struct Iface {
    pub mac: [u8; 6],
    pub ipv4: [u8; 4],
}

pub struct ReplyFrame {
    pub bytes: [u8; HDR_LEN + PACKET_LEN],
}

// Consume an inbound ARP packet payload. Apply the learn policy
// before touching the cache: refresh a matching binding, reject a
// MAC rebind, and insert a new binding only when the packet is
// solicited (a reply to our own outstanding request, or a request
// that directly targets us). If the packet is a request for our own
// IPv4 address, build and return the reply frame so the caller can
// hand it to the NIC client. Returns `None` when the packet does
// not require a response.
pub fn on_inbound(iface: &Iface, cache: &mut Cache, payload: &[u8]) -> Option<ReplyFrame> {
    let pkt = ArpPacket::parse(payload)?;
    let solicited = cache.is_pending(pkt.sender_ip)
        || (pkt.oper == OPER_REQUEST && pkt.target_ip == iface.ipv4);
    match crate::arp::cache::decide(cache.lookup(&pkt.sender_ip), pkt.sender_mac, solicited) {
        crate::arp::cache::Learn::Insert | crate::arp::cache::Learn::Refresh => {
            cache.insert(pkt.sender_ip, pkt.sender_mac);
        }
        crate::arp::cache::Learn::Reject => {}
    }
    if pkt.oper != OPER_REQUEST || pkt.target_ip != iface.ipv4 {
        return None;
    }
    let mut bytes = [0u8; HDR_LEN + PACKET_LEN];
    EthHeader { dst: pkt.sender_mac, src: iface.mac, ethertype: ETHERTYPE_ARP }
        .write(&mut bytes[..HDR_LEN]);
    ArpPacket {
        oper: OPER_REPLY,
        sender_mac: iface.mac,
        sender_ip: iface.ipv4,
        target_mac: pkt.sender_mac,
        target_ip: pkt.sender_ip,
    }
    .write(&mut bytes[HDR_LEN..]);
    Some(ReplyFrame { bytes })
}

// Build an ARP request frame for `target_ipv4`. Broadcast on the
// L2; the answer arrives as an `on_inbound(OPER_REPLY)` callback
// which seeds the cache.
pub fn build_request(iface: &Iface, target_ipv4: [u8; 4]) -> [u8; HDR_LEN + PACKET_LEN] {
    let mut bytes = [0u8; HDR_LEN + PACKET_LEN];
    EthHeader { dst: MAC_BROADCAST, src: iface.mac, ethertype: ETHERTYPE_ARP }
        .write(&mut bytes[..HDR_LEN]);
    ArpPacket {
        oper: OPER_REQUEST,
        sender_mac: iface.mac,
        sender_ip: iface.ipv4,
        target_mac: [0u8; 6],
        target_ip: target_ipv4,
    }
    .write(&mut bytes[HDR_LEN..]);
    bytes
}
