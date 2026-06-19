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

pub mod init;
pub mod io;
pub mod read_write;

pub use init::{init, init_port};
pub use io::{
    is_data_ready, is_tx_empty, read_byte_direct, read_reg, write_byte_timeout, write_reg,
};
pub use read_write::{
    available, available_from_port, is_port_initialized, module_is_initialized, read_byte,
    read_byte_direct_from_port, read_byte_from_port, write_byte, write_byte_to_port, write_str,
    write_str_to_port,
};
