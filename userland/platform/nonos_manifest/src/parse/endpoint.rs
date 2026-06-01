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

use crate::endpoint::{Endpoint, EndpointKind};

pub fn finish_endpoint(parts: (String, u32, String)) -> Result<Endpoint, String> {
    let kind = match parts.0.as_str() {
        "service" => EndpointKind::Service,
        "reply" => EndpointKind::Reply,
        other => return Err(format!("bad endpoint kind: {other}")),
    };
    if parts.2.is_empty() {
        return Err("endpoint missing name".to_string());
    }
    Ok(Endpoint { kind, port: parts.1, name: parts.2 })
}
