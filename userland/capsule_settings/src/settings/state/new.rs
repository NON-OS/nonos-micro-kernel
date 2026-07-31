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

use nonos_policy_proto::Category;

use crate::wifi::{NetStatus, ScanNetwork, ScanStats, WifiInterface};

use super::cache::FieldValue;
use super::edit_buffer::EditBuffer;
use super::state::{State, WifiConnect, WifiScan, FIELD_SLOTS, WIFI_MAX, WIFI_NET_MAX};
use super::status::Status;

pub fn new() -> State {
    State {
        policy_port: 0,
        policy_ready: false,
        category: Category::User,
        cursor: [0, 0, 0],
        scroll_top: [0, 0, 0],
        values: [FieldValue::Unknown; FIELD_SLOTS],
        editing: false,
        edit: EditBuffer::empty(),
        status: Status::idle(),
        wifi_active: false,
        wifi_adapters: [WifiInterface::default(); WIFI_MAX],
        wifi_adapter_count: 0,
        wifi_cursor: 0,
        wifi_networks: [ScanNetwork::default(); WIFI_NET_MAX],
        wifi_network_count: 0,
        wifi_scan: WifiScan::Idle,
        wifi_stage: None,
        wifi_stats: ScanStats::default(),
        wifi_pass_active: false,
        wifi_pass: EditBuffer::empty(),
        wifi_connect: WifiConnect::Idle,
        wifi_datapath: None,
        wifi_net: NetStatus::NoService,
        win_w: crate::settings::manifest::WIDTH,
        win_h: crate::settings::manifest::HEIGHT,
    }
}
