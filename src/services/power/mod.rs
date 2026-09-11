//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

mod proxy;

pub mod device;
pub mod kbd_backlight;
pub mod types;

use std::sync::Arc;

use anyhow::Result;
use futures_util::StreamExt;
use tokio_util::sync::CancellationToken;
use zbus::Connection;

use crate::{
    core::property::Property,
    services::power::{device::Device, kbd_backlight::KbdBacklight, proxy::UPowerProxy},
};

pub struct PowerService {
    connection: Connection,
    upower_proxy: UPowerProxy<'static>,
    cancellation_token: CancellationToken,

    pub devices: Property<Vec<Arc<Device>>>,
    pub kbd_backlights: Property<Vec<Arc<KbdBacklight>>>,
    pub display_device: Property<Arc<Device>>,
    pub on_battery: Property<bool>,
    pub lid_is_closed: Property<bool>,
    pub lid_is_present: Property<bool>,
}

impl PowerService {
    pub async fn new() -> Result<Self> {
        let connection = Connection::system().await?;
        let upower_proxy = UPowerProxy::new(&connection).await?;
        let cancellation_token = CancellationToken::new();

        let devices = {
            let device_paths = upower_proxy.enumerate_devices().await?;
            let mut devices = Vec::with_capacity(device_paths.len());

            for path in device_paths {
                let device =
                    Device::new(&connection, path, cancellation_token.child_token()).await?;
                devices.push(Arc::new(device));
            }

            Property::new(devices)
        };

        let kbd_backlights = {
            let kbd_backlight_paths = upower_proxy.enumerate_kbd_backlights().await?;
            let mut kbd_backlights = Vec::with_capacity(kbd_backlight_paths.len());

            for path in kbd_backlight_paths {
                let kbd_backlight =
                    KbdBacklight::new(&connection, path, cancellation_token.child_token()).await?;
                kbd_backlights.push(Arc::new(kbd_backlight));
            }

            Property::new(kbd_backlights)
        };

        let display_device = {
            let path = upower_proxy.get_display_device().await?;
            let display_device =
                Arc::new(Device::new(&connection, path, cancellation_token.child_token()).await?);
            Property::new(display_device)
        };

        let on_battery = Property::new(upower_proxy.on_battery().await?);
        let lid_is_closed = Property::new(upower_proxy.lid_is_closed().await?);
        let lid_is_present = Property::new(upower_proxy.lid_is_present().await?);

        {
            let connection = connection.clone();
            let cancellation_token = cancellation_token.child_token();

            let devices = devices.clone();
            let on_battery = on_battery.clone();
            let lid_is_closed = lid_is_closed.clone();
            let lid_is_present = lid_is_present.clone();

            let mut device_added_stream = upower_proxy.receive_device_added().await?;
            let mut device_removed_stream = upower_proxy.receive_device_removed().await?;

            let mut on_battery_stream = upower_proxy.receive_on_battery_changed().await;
            let mut lid_is_closed_stream = upower_proxy.receive_lid_is_closed_changed().await;
            let mut lid_is_present_stream = upower_proxy.receive_lid_is_present_changed().await;

            relm4::spawn(async move {
                loop {
                    async_select::select! {
                        Some(change) = device_added_stream.next() => {
                            if let Ok(args) = change.args() {
                                let path = args.device;
                                let device = Device::new(&connection, path, cancellation_token.child_token()).await.unwrap();

                                let mut devices_vec = devices.read();
                                devices_vec.push(Arc::new(device));
                                devices.write_unconditional(devices_vec);
                            }
                        }
                        Some(change) = device_removed_stream.next() => {
                            if let Ok(args) = change.args() {
                                let path = args.device;

                                let mut devices_vec = devices.read();
                                devices_vec.retain(|device| {
                                    device.path != path
                                });
                                devices.write_unconditional(devices_vec);
                            }
                        }
                        Some(change) = on_battery_stream.next() => {
                            if let Ok(value) = change.get().await {
                                on_battery.write(value);
                            }
                        }
                        Some(change) = lid_is_closed_stream.next() => {
                            if let Ok(value) = change.get().await {
                                lid_is_closed.write(value);
                            }
                        }
                        Some(change) = lid_is_present_stream.next() => {
                            if let Ok(value) = change.get().await {
                                lid_is_present.write(value);
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
            connection,
            upower_proxy,
            cancellation_token,

            devices,
            kbd_backlights,
            display_device,
            on_battery,
            lid_is_closed,
            lid_is_present,
        })
    }
}

impl Drop for PowerService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
