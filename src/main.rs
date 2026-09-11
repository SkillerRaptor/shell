//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

mod app;
mod bar;
mod core;
mod notifications;
mod osd;
mod services;

use relm4::RelmApp;

use crate::app::AppModel;

fn main() {
    let app = RelmApp::new("dev.skillerraptor.shell");
    app.visible_on_activate(false).run_async::<AppModel>(());
}
