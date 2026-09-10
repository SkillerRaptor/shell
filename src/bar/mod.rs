//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

use gtk4_layer_shell::LayerShell;
use relm4::{SimpleComponent, gtk::prelude::*, prelude::*};

pub struct BarModel {}

#[relm4::component(pub)]
impl SimpleComponent for BarModel {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Window {
            init_layer_shell: (),
            set_layer: gtk4_layer_shell::Layer::Top,
            auto_exclusive_zone_enable: (),

            set_anchor: (gtk4_layer_shell::Edge::Top, true),
            set_anchor: (gtk4_layer_shell::Edge::Bottom, false),
            set_anchor: (gtk4_layer_shell::Edge::Left, true),
            set_anchor: (gtk4_layer_shell::Edge::Right, true),

            set_margin: (gtk4_layer_shell::Edge::Top, 0),
            set_margin: (gtk4_layer_shell::Edge::Bottom, 0),
            set_margin: (gtk4_layer_shell::Edge::Left, 16),
            set_margin: (gtk4_layer_shell::Edge::Right, 16),

            set_default_height: 32,

            set_title: None,
            set_decorated: false,
            set_visible: true,

            gtk::CenterBox {
                #[wrap(Some)]
                set_start_widget = &gtk::Box {},

                #[wrap(Some)]
                set_center_widget = &gtk::Box {},

                #[wrap(Some)]
                set_end_widget = &gtk::Box {}
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
