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

mod default_net;
mod empty_rail;
mod filter_rails;
mod hydrate;
mod live_view;
mod new;
mod rail_allowed;
mod record_tx;
mod types;

pub use default_net::default_net;
pub use hydrate::hydrate;
pub use live_view::needs_live_data;
pub use new::new_state;
pub use record_tx::record_tx;
pub use types::{
    Rail, State, MAX_RAILS, MAX_STAKE, SEND_FIELD_AMOUNT, SEND_FIELD_NONCE, SEND_FIELD_TO,
    VIEW_APPROVALS, VIEW_HOME, VIEW_NOX, VIEW_PROOF, VIEW_RECEIVE, VIEW_SEND, VIEW_SHIELD,
    VIEW_SHIELDED, VIEW_SIGN, VIEW_SWAP, VIEW_UNSHIELD,
};
