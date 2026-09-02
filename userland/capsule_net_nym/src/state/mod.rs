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

mod ack_key;
mod authority;
mod bootstrap;
mod bootstrap_mix;
mod credential;
mod directory;
mod directory_gateway;
mod gateway;
mod identity;
mod replay;
mod session;
mod shared_key;
mod surb;
mod surb_id;
mod surb_tag;
mod surb_types;
mod table;
mod timing;

pub use ack_key::ack_key;
pub use authority::{install as install_authority, trusted as trusted_authority};
pub use bootstrap::{bootstrap_gateway, BOOTSTRAP_GATEWAYS};
pub use bootstrap_mix::{bootstrap_route, BOOTSTRAP_MIXNODES, PER_LAYER};
pub use credential::{
    install as install_credential, material as credential_material, CredentialError,
};
pub use directory::{get as directory_source, install as install_directory_source};
pub use directory_gateway::{directory_gateway, directory_gateway_count, directory_exit_count};
pub use gateway::{Gateway, Transport};
pub use identity::{client_identity, set_client_identity, Identity as ClientIdentity};
pub use session::Session;
pub use shared_key::{clear_gateway_shared_key, gateway_shared_key, set_gateway_shared_key};
pub use surb::{consume as consume_surb, create as create_surb, default_ttl_ms as surb_ttl_ms};
pub use table::{TableError, TABLE};
pub use timing::{cover_due, install as install_timing, next_cover_ms, policy as timing_policy};
