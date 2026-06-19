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

//! Kernel-side glue for the userland marketplace capsule. The
//! kernel embeds the binary, spawns it, registers the
//! `market.index` service inbox, and exposes a thin IPC client.
//! It does not parse marketplace JSON, it does not know NOX or
//! pricing policy — those live in the userland capsule and the
//! `marketplace_abi` wire-form crate.

mod capability;
pub mod client;
mod embed;
mod error;
mod protocol;
mod spawn;
mod state;

pub use client::{get_app, get_release, healthcheck, install_ready, list_apps, load_index};
pub use error::MarketError;
pub use spawn::{spawn_market_capsule, SpawnError};
pub use state::shared_state;
