//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

use relm4::{gtk::prelude::*, prelude::*};

use crate::{
    bar::BarModel,
    services::{power::PowerService, system_info::SystemInfoService},
};

pub struct AppModel {
    _power_service: PowerService,
    _system_info_service: SystemInfoService,
    _bar: Controller<BarModel>,
}

#[relm4::component(pub, async)]
impl AsyncComponent for AppModel {
    type Input = ();
    type Output = ();
    type Init = ();
    type CommandOutput = ();

    view! {
        gtk::Window {
            set_visible: false,
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        // TODO: Error Handling
        let power_service = PowerService::new().await.unwrap();
        let system_info_service = SystemInfoService::new().await.unwrap();

        let bar = BarModel::builder().launch(()).detach();

        let model = Self {
            _power_service: power_service,
            _system_info_service: system_info_service,
            _bar: bar,
        };

        let widgets = view_output!();
        AsyncComponentParts { model, widgets }
    }
}
