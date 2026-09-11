//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

use relm4::{gtk::prelude::*, prelude::*};

use crate::{
    bar::BarModel,
    services::{battery::BatteryService, network::NetworkService, system_info::SystemInfoService},
};

pub struct AppModel {
    _battery_service: BatteryService,
    _network_service: NetworkService,
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
        let battery_service = BatteryService::new().await.unwrap();
        let network_service = NetworkService::new().await.unwrap();
        let system_info_service = SystemInfoService::new().await.unwrap();

        let mut devices = network_service.devices.subscribe();
        let mut all_devices = network_service.all_devices.subscribe();
        let mut networking_enabled = network_service.networking_enabled.subscribe();
        let mut wireless_enabled = network_service.wireless_enabled.subscribe();
        let mut wireless_hardware_enabled = network_service.wireless_hardware_enabled.subscribe();
        let mut wwan_enabled = network_service.wwan_enabled.subscribe();
        let mut wwan_hardware_enabled = network_service.wwan_hardware_enabled.subscribe();
        let mut active_connections = network_service.active_connections.subscribe();
        let mut primary_connection = network_service.primary_connection.subscribe();
        let mut primary_connection_type = network_service.primary_connection_type.subscribe();
        let mut metered = network_service.metered.subscribe();
        let mut activating_connection = network_service.activating_connection.subscribe();
        let mut state = network_service.state.subscribe();
        let mut connectivity = network_service.connectivity.subscribe();
        let mut connectivity_check_available =
            network_service.connectivity_check_available.subscribe();
        let mut connectivity_check_enabled = network_service.connectivity_check_enabled.subscribe();
        relm4::spawn(async move {
            loop {
                async_select::select! {
                    Some(value) = devices.next() => {
                        println!("devices: {:#?}", value);
                    }
                    Some(value) = all_devices.next() => {
                        println!("all_devices: {:#?}", value);
                    }
                    Some(value) = networking_enabled.next() => {
                        println!("networking_enabled: {:#?}", value);
                    }
                    Some(value) = wireless_enabled.next() => {
                        println!("wireless_enabled: {:#?}", value);
                    }
                    Some(value) = wireless_hardware_enabled.next() => {
                        println!("wireless_hardware_enabled: {:#?}", value);
                    }
                    Some(value) = wwan_enabled.next() => {
                        println!("wwan_enabled: {:#?}", value);
                    }
                    Some(value) = wwan_hardware_enabled.next() => {
                        println!("wwan_hardware_enabled: {:#?}", value);
                    }
                    Some(value) = active_connections.next() => {
                        println!("active_connections: {:#?}", value);
                    }
                    Some(value) = primary_connection.next() => {
                        println!("primary_connection: {:#?}", value);
                    }
                    Some(value) = primary_connection_type.next() => {
                        println!("primary_connection_type: {:#?}", value);
                    }
                    Some(value) = metered.next() => {
                        println!("metered: {:#?}", value);
                    }
                    Some(value) = activating_connection.next() => {
                        println!("activating_connection: {:#?}", value);
                    }
                    Some(value) = state.next() => {
                        println!("state: {:#?}", value);
                    }
                    Some(value) = connectivity.next() => {
                        println!("connectivity: {:#?}", value);
                    }
                    Some(value) = connectivity_check_available.next() => {
                        println!("connectivity_check_available: {:#?}", value);
                    }
                    Some(value) = connectivity_check_enabled.next() => {
                        println!("connectivity_check_enabled: {:#?}", value);
                    }
                }
            }
        });

        let bar = BarModel::builder().launch(()).detach();

        let model = Self {
            _battery_service: battery_service,
            _network_service: network_service,
            _system_info_service: system_info_service,
            _bar: bar,
        };

        let widgets = view_output!();
        AsyncComponentParts { model, widgets }
    }
}
