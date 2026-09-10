//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

use relm4::{SimpleComponent, gtk::prelude::*, prelude::*};

pub struct AppModel {}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Input = ();
    type Output = ();
    type Init = ();

    view! {
        gtk::Window {
            set_visible: false,
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
