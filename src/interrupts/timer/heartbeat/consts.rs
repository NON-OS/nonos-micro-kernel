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

pub(super) const X0: u32 = 6;
pub(super) const Y0: u32 = 30;
pub(super) const SIZE: u32 = 16;

// 1 kHz-ish palette so a photographed frame lands on a definite color.
pub(super) const PALETTE: [u32; 6] = [
    0xFF00_E5FF, // cyan
    0xFF00_D060, // green
    0xFFFF_D000, // gold
    0xFFFF_6000, // orange
    0xFFE0_2040, // red
    0xFFB0_50FF, // violet
];

pub(super) const BAR_X0: u32 = 6;
pub(super) const BAR_H: u32 = 6;
pub(super) const BAR_MAX: u32 = 600;
