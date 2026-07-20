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

//! MAC initialisation: the register-operation engine that runs the rtw88
//! `rtw8821c_mac_init` program, and (once the chip is proven on silicon to reach
//! this stage) the init table itself. The engine is proven against a modeled
//! device in `rtl8821ce_proofs`.

pub mod op;
mod run;
mod tables;
mod trx;

pub use run::run_mac_table;
pub use tables::MAC_INIT;
pub use trx::{init_trx_cfg, reset_trx_dma};
