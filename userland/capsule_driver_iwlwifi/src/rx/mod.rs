// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The receive path: reading the firmware's responses and notifications. A
//! command issued through the host-command queue is answered by a packet the
//! firmware posts here, matched back by its sequence. `packet` frames a
//! received buffer, `ring` tracks the read index against the firmware's write
//! index, and `recv` ties them into receiving the next packet.

pub mod packet;
pub mod recv;
pub mod ring;
