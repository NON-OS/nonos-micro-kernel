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

//! The RTL8821CE firmware image: its header (which the download reads to split
//! the body into the DMEM, IMEM and EMEM sections the 8051 expects) and, later,
//! the download engine that programs those sections into the card.

pub mod ddma;
pub mod dma;
mod download;
pub mod header;
pub mod prep;
pub mod regs;
pub mod rsvd;
pub mod sections;
pub mod txdesc;

pub use download::{download, DownloadError};
pub use sections::MAX_CHUNK;
