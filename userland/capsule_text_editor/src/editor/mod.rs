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

mod activity_bar;
mod app;
mod autoclose;
mod backspace;
mod byte_at;
mod canvas;
mod caret_nav;
mod clamp_scroll;
mod click_caret;
mod ctrl_copy;
mod ctrl_cut;
mod ctrl_export;
mod ctrl_open;
mod ctrl_paste;
mod ctrl_save;
mod delete;
mod doc_pos;
mod edit;
mod event;
mod find;
mod find_key;
mod follow_caret;
mod highlight;
mod indent;
mod insert;
mod insert_newline;
mod language;
mod layout;
mod line_bounds;
mod line_ops;
mod manifest;
mod max_scroll;
mod menubar;
mod mode;
mod notify;
mod on_ctrl;
mod on_ctrl_nav;
mod on_nav;
mod paint;
mod path_prompt;
mod position_at;
mod reflow;
mod replace;
mod resolve_owner_pid;
mod ribbon;
mod sb_entry;
mod sb_menu;
mod select_word;
mod selection;
mod shell;
mod sidebar;
mod state;
mod state_new;
mod statusbar;
mod tabbar;
mod theme;
mod toggle_comment;
mod tree;
mod visual_lines;
mod word_nav;
mod ws_event;
mod ws_menu;
mod ws_menubar;
mod ws_open;
mod ws_paint;
mod ws_pointer;
mod ws_wheel;

pub use app::Editor;
