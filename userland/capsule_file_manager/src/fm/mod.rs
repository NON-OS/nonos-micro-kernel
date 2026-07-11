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

mod app;
mod clipboard;
mod clipboard_paste;
mod duplicate;
mod entries;
mod event;
mod event_actions;
mod event_browse;
mod event_dispatch;
mod event_mode;
mod event_modes;
mod event_mouse;
mod event_move;
mod event_open;
mod event_parent;
mod file_color;
mod file_ext;
mod file_kind;
mod filetype;
mod filter;
mod fmt_time;
mod help;
mod human_size;
mod layout;
mod manifest;
mod open_with;
mod paint;
mod paint_clip;
mod paint_footer;
mod paint_header;
mod paint_metrics;
mod paint_right;
mod paint_rows;
mod perms;
mod preview;
mod preview_clip;
mod preview_hex;
mod preview_info;
mod preview_is_binary;
mod preview_key;
mod preview_paint;
mod preview_text;
mod prompt;
mod prompt_commit;
mod prompt_run_op;
mod prompt_start;
mod refresh;
mod refresh_meta;
mod scroll;
mod selection;
mod selection_acting;
mod selection_clear;
mod selection_is_selected;
mod selection_select_all;
mod sort_label;
mod sort_next;
mod state;
mod state_new;
mod theme;
mod view;
mod view_sort;
mod view_sort_mode;
mod view_sort_name;

pub use app::FileManager;
