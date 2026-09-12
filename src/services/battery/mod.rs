//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

mod proxy;

pub mod battery;
pub mod peripheral;
pub mod physical_battery;
pub mod types;

use std::sync::Arc;

use anyhow::Result;
use futures_util::StreamExt;
use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::{
    core::property::Property,
    services::battery::{
        battery::Battery,
        peripheral::Peripheral,
        physical_battery::PhysicalBattery,
        proxy::{DeviceProxy, UPowerProxy},
        types::Kind,
    },
};

pub struct BatteryService {
    _connection: Connection,
    _upower_proxy: UPowerProxy<'static>,
    cancellation_token: CancellationToken,

    pub battery: Property<Arc<Battery>>,
    pub physical_batteries: Property<Vec<Arc<PhysicalBattery>>>,
    pub peripherals: Property<Vec<Arc<Peripheral>>>,
    pub on_battery: Property<bool>,
}

impl BatteryService {
    pub async fn new() -> Result<Self> {
        let connection = Connection::system().await?;
        let upower_proxy = UPowerProxy::new(&connection).await?;
        let cancellation_token = CancellationToken::new();

        let battery = {
            let path = upower_proxy.get_display_device().await?;
            let battery = Battery::new(&connection, path, cancellation_token.child_token()).await?;
            Property::new(Arc::new(battery))
        };

        let (physical_batteries, peripherals) = {
            let device_paths = upower_proxy.enumerate_devices().await?;

            let mut physical_batteries: Vec<Arc<PhysicalBattery>> = Vec::new();
            let mut peripherals = Vec::new();

            for path in device_paths {
                let device_proxy = DeviceProxy::builder(&connection)
                    .path(path.clone())?
                    .build()
                    .await?;

                let kind = device_proxy.kind().await?;

                match kind {
                    Kind::Battery => {
                        let physical_battery = PhysicalBattery::new(
                            &connection,
                            path,
                            cancellation_token.child_token(),
                        )
                        .await?;

                        physical_batteries.push(Arc::new(physical_battery));
                    }
                    Kind::Mouse
                    | Kind::Keyboard
                    | Kind::Pen
                    | Kind::Headset
                    | Kind::Speakers
                    | Kind::Headphones => {
                        let peripheral = Peripheral::new(&connection, path).await?;
                        peripherals.push(Arc::new(peripheral));
                    }
                    _ => {}
                }
            }

            (
                Property::new(physical_batteries),
                Property::new(peripherals),
            )
        };

        let on_battery = Property::new(upower_proxy.on_battery().await?);

        {
            let connection = connection.clone();
            let cancellation_token = cancellation_token.child_token();

            let physical_batteries = physical_batteries.clone();
            let peripherals = peripherals.clone();
            let on_battery = on_battery.clone();

            let mut device_added_stream = upower_proxy.receive_device_added().await?;
            let mut device_removed_stream = upower_proxy.receive_device_removed().await?;

            let mut on_battery_stream = upower_proxy.receive_on_battery_changed().await;

            relm4::spawn(async move {
                loop {
                    async_select::select! {
                        Some(change) = device_added_stream.next() => {
                            if let Ok(args) = change.args() {
                                let path: OwnedObjectPath = args.device.into();

                                let device_proxy = DeviceProxy::builder(&connection)
                                    .path(path.clone())
                                    .unwrap()
                                    .build()
                                    .await
                                    .unwrap();

                                let kind = device_proxy.kind().await.unwrap();

                                match kind {
                                    Kind::Battery => {
                                        let physical_battery = PhysicalBattery::new(
                                            &connection,
                                            path,
                                            cancellation_token.child_token(),
                                        )
                                        .await
                                        .unwrap();

                                        let mut physical_batteries_vec = physical_batteries.read();
                                        physical_batteries_vec.push(Arc::new(physical_battery));
                                        physical_batteries.write_unconditional(physical_batteries_vec);
                                    }
                                    Kind::Mouse
                                    | Kind::Keyboard
                                    | Kind::Pen
                                    | Kind::Headset
                                    | Kind::Speakers
                                    | Kind::Headphones => {
                                        let peripheral = Peripheral::new(&connection, path).await.unwrap();

                                        let mut peripherals_vec = peripherals.read();
                                        peripherals_vec.push(Arc::new(peripheral));
                                        peripherals.write_unconditional(peripherals_vec);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Some(change) = device_removed_stream.next() => {
                            if let Ok(args) = change.args() {
                                let path: OwnedObjectPath = args.device.into();

                                let device_proxy = DeviceProxy::builder(&connection)
                                    .path(path.clone())
                                    .unwrap()
                                    .build()
                                    .await
                                    .unwrap();

                                let kind = device_proxy.kind().await.unwrap();

                                match kind {
                                    Kind::Battery => {
                                        let mut physical_batteries_vec = physical_batteries.read();
                                        physical_batteries_vec.retain(|physical_battery| {
                                            physical_battery.path != path
                                        });
                                        physical_batteries.write_unconditional(physical_batteries_vec);
                                    }
                                    Kind::Mouse
                                    | Kind::Keyboard
                                    | Kind::Pen
                                    | Kind::Headset
                                    | Kind::Speakers
                                    | Kind::Headphones => {
                                        let mut peripherals_vec = peripherals.read();
                                        peripherals_vec.retain(|peripheral| {
                                            peripheral.path != path
                                        });
                                        peripherals.write_unconditional(peripherals_vec);
                                    }
                                    _ => {}
                                }
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

            battery,
            physical_batteries,
            peripherals,
            on_battery,
        })
    }
}

impl Drop for BatteryService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
