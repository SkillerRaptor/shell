//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

mod proxy;

pub mod battery;
pub mod display_device;
pub mod types;

use std::sync::Arc;

use anyhow::Result;
use futures_util::StreamExt;
use tokio_util::sync::CancellationToken;
use zbus::Connection;

use crate::{
    core::property::Property,
    services::battery::{
        battery::Battery,
        display_device::DisplayDevice,
        proxy::UPowerProxy,
        types::Type,
    },
};

pub struct BatteryService {
    _connection: Connection,
    _upower_proxy: UPowerProxy<'static>,
    cancellation_token: CancellationToken,

    pub display_device: Property<Arc<DisplayDevice>>,
    pub batteries: Property<Vec<Arc<Battery>>>,
    pub on_battery: Property<bool>,
}

impl BatteryService {
    pub async fn new() -> Result<Self> {
        let connection = Connection::system().await?;
        let upower_proxy = UPowerProxy::new(&connection).await?;
        let cancellation_token = CancellationToken::new();

        let display_device = {
            let path = upower_proxy.get_display_device().await?;
            let display_device =
                DisplayDevice::new(&connection, path, cancellation_token.child_token()).await?;
            Property::new(Arc::new(display_device))
        };

        let batteries = {
            let device_paths = upower_proxy.enumerate_devices().await?;

            let mut batteries = Vec::with_capacity(device_paths.len());
            for path in device_paths {
                let battery =
                    Battery::new(&connection, path, cancellation_token.child_token()).await?;
                if battery.r#type.read() == Type::Battery {
                    batteries.push(Arc::new(battery));
                }
            }

            Property::new(batteries)
        };

        let on_battery = Property::new(upower_proxy.on_battery().await?);

        {
            let connection = connection.clone();
            let cancellation_token = cancellation_token.child_token();

            let batteries = batteries.clone();
            let on_battery = on_battery.clone();

            let mut device_added_stream = upower_proxy.receive_device_added().await?;
            let mut device_removed_stream = upower_proxy.receive_device_removed().await?;

            let mut on_battery_stream = upower_proxy.receive_on_battery_changed().await;

            relm4::spawn(async move {
                loop {
                    async_select::select! {
                        Some(change) = device_added_stream.next() => {
                            if let Ok(args) = change.args() {
                                let path = args.device.into();
                                let battery = Battery::new(&connection, path, cancellation_token.child_token()).await.unwrap();

                                let mut batteries_vec = batteries.read();
                                batteries_vec.push(Arc::new(battery));
                                batteries.write_unconditional(batteries_vec);
                            }
                        }
                        Some(change) = device_removed_stream.next() => {
                            if let Ok(args) = change.args() {
                                let path = args.device.into();

                                let mut batteries_vec = batteries.read();
                                batteries_vec.retain(|battery| {
                                    battery.path != path
                                });
                                batteries.write_unconditional(batteries_vec);
                            }
                        }
                        Some(change) = on_battery_stream.next() => {
                            if let Ok(value) = change.get().await {
                                on_battery.write(value);
                            }
                        }
                        _ = cancellation_token.cancelled() => {
                            break;
                        }
                    }
                }
            });
        }

        Ok(Self {
            _connection: connection,
            _upower_proxy: upower_proxy,
            cancellation_token,

            display_device,
            batteries,
            on_battery,
        })
    }
}

impl Drop for BatteryService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
