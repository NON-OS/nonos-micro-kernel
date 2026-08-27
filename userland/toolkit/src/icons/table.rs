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

/// One 8-bit coverage mask per icon, ordered to match `IconId`. The host test
/// compares every entry against the file `IconId::name` points at, so the
/// ordinal indexing below is proven rather than assumed.
pub(super) const MASKS: [&[u8]; 42] = [
    include_bytes!("../../../assets/icons/about.a8"),
    include_bytes!("../../../assets/icons/audio_player.a8"),
    include_bytes!("../../../assets/icons/browser.a8"),
    include_bytes!("../../../assets/icons/calc.a8"),
    include_bytes!("../../../assets/icons/clock.a8"),
    include_bytes!("../../../assets/icons/editor.a8"),
    include_bytes!("../../../assets/icons/files.a8"),
    include_bytes!("../../../assets/icons/fs_file.a8"),
    include_bytes!("../../../assets/icons/fs_folder.a8"),
    include_bytes!("../../../assets/icons/image_viewer.a8"),
    include_bytes!("../../../assets/icons/processes.a8"),
    include_bytes!("../../../assets/icons/settings.a8"),
    include_bytes!("../../../assets/icons/snake.a8"),
    include_bytes!("../../../assets/icons/terminal.a8"),
    include_bytes!("../../../assets/icons/video_player.a8"),
    include_bytes!("../../../assets/icons/wallet.a8"),
    include_bytes!("../../../assets/icons/settings_appearance.a8"),
    include_bytes!("../../../assets/icons/settings_developer.a8"),
    include_bytes!("../../../assets/icons/settings_general.a8"),
    include_bytes!("../../../assets/icons/settings_network.a8"),
    include_bytes!("../../../assets/icons/settings_privacy.a8"),
    include_bytes!("../../../assets/icons/settings_search.a8"),
    include_bytes!("../../../assets/icons/settings_security.a8"),
    include_bytes!("../../../assets/icons/settings_sound.a8"),
    include_bytes!("../../../assets/icons/settings_storage.a8"),
    include_bytes!("../../../assets/icons/settings_updates.a8"),
    include_bytes!("../../../assets/icons/settings_wifi.a8"),
    include_bytes!("../../../assets/icons/pm_overview.a8"),
    include_bytes!("../../../assets/icons/pm_cpu.a8"),
    include_bytes!("../../../assets/icons/pm_memory.a8"),
    include_bytes!("../../../assets/icons/pm_authority.a8"),
    include_bytes!("../../../assets/icons/pm_security.a8"),
    include_bytes!("../../../assets/icons/game_trophy.a8"),
    include_bytes!("../../../assets/icons/game_crown.a8"),
    include_bytes!("../../../assets/icons/game_gauge.a8"),
    include_bytes!("../../../assets/icons/game_heart.a8"),
    include_bytes!("../../../assets/icons/game_lotus.a8"),
    include_bytes!("../../../assets/icons/game_bolt.a8"),
    include_bytes!("../../../assets/icons/game_blocks.a8"),
    include_bytes!("../../../assets/icons/game_stopwatch.a8"),
    include_bytes!("../../../assets/icons/game_target.a8"),
    include_bytes!("../../../assets/icons/game_lock.a8"),
    include_bytes!("../../../assets/icons/calc_basic.a8"),
    include_bytes!("../../../assets/icons/calc_scientific.a8"),
    include_bytes!("../../../assets/icons/calc_programmer.a8"),
    include_bytes!("../../../assets/icons/calc_convert.a8"),
    include_bytes!("../../../assets/icons/calc_history.a8"),
];
