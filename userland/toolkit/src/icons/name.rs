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


use super::id::IconId;

impl IconId {
    pub fn name(self) -> &'static str {
        match self {
            IconId::About => "about",
            IconId::AudioPlayer => "audio_player",
            IconId::Browser => "browser",
            IconId::Calc => "calc",
            IconId::Clock => "clock",
            IconId::Editor => "editor",
            IconId::Files => "files",
            IconId::FsFile => "fs_file",
            IconId::FsFolder => "fs_folder",
            IconId::ImageViewer => "image_viewer",
            IconId::Processes => "processes",
            IconId::Settings => "settings",
            IconId::Snake => "snake",
            IconId::Terminal => "terminal",
            IconId::VideoPlayer => "video_player",
            IconId::Wallet => "wallet",
            IconId::SettingsAppearance => "settings_appearance",
            IconId::SettingsDeveloper => "settings_developer",
            IconId::SettingsGeneral => "settings_general",
            IconId::SettingsNetwork => "settings_network",
            IconId::SettingsPrivacy => "settings_privacy",
            IconId::SettingsSearch => "settings_search",
            IconId::SettingsSecurity => "settings_security",
            IconId::SettingsSound => "settings_sound",
            IconId::SettingsStorage => "settings_storage",
            IconId::SettingsUpdates => "settings_updates",
            IconId::SettingsWifi => "settings_wifi",
            IconId::PmOverview => "pm_overview",
            IconId::PmCpu => "pm_cpu",
            IconId::PmMemory => "pm_memory",
            IconId::PmAuthority => "pm_authority",
            IconId::PmSecurity => "pm_security",
        }
    }
}
