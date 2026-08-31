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
mod activity_event;
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
mod event_editor;
mod find;
mod find_key;
mod follow_caret;
mod highlight;
mod home;
mod indent;
mod info_text;
mod insert;
mod insert_newline;
mod language;
mod layout;
mod line_bounds;
mod line_ops;
mod list_ops;
mod list_renumber;
mod manifest;
mod max_scroll;
mod menubar;
mod mode;
mod notify;
mod on_ctrl;
mod on_ctrl_nav;
mod on_nav;
mod pagebreak;
mod pagebreak_mark;
mod paint;
mod paint_editor;
mod panel;
mod panel_geom;
mod panel_paint;
mod panel_press;
mod panel_rows;
mod path_prompt;
mod position_at;
mod reflow;
mod replace;
mod resolve_owner_pid;
mod ribbon;
mod sb_entry;
mod screen;
mod sb_menu;
mod select_word;
mod selection;
mod settings;
mod shell;
mod sidebar;
mod specials;
mod state;
mod state_new;
mod statusbar;
mod tabbar;
mod table_edit;
mod table_menu;
mod table_ops;
mod table_paint;
mod table_rules;
mod theme;
mod toggle_comment;
mod tree;
mod unsupported;
mod visual_lines;
mod widget;
mod word_nav;
mod wordcount;
mod ws_event;
mod ws_info;
mod ws_menu;
mod ws_menubar;
mod ws_open;
mod ws_paint;
mod ws_pointer;
mod ws_wheel;

pub use app::Editor;
