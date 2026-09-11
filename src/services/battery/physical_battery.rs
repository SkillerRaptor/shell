//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

use std::time::Duration;

use anyhow::Result;
use futures_util::StreamExt;
use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::{
    core::property::Property,
    services::battery::{
        proxy::DeviceProxy,
        types::{Kind, State},
    },
};

pub struct PhysicalBattery {
    pub(super) path: OwnedObjectPath,
    cancellation_token: CancellationToken,
    device_proxy: DeviceProxy<'static>,

    pub native_path: Property<String>,
    pub vendor: Property<String>,
    pub model: Property<String>,
    pub serial: Property<String>,
    pub kind: Property<Kind>,
    pub time_to_empty: Property<Duration>,
    pub time_to_full: Property<Duration>,
    pub percentage: Property<f64>,
    pub temperature: Property<f64>,
    pub state: Property<State>,
    pub is_rechargeable: Property<bool>,
    pub capacity: Property<f64>,
    pub charge_start_threshold: Property<u32>,
    pub charge_end_threshold: Property<u32>,
    pub charge_threshold_enabled: Property<bool>,
    pub charge_threshold_supported: Property<bool>,
    pub charge_threshold_settings_supported: Property<u32>,
}

impl PhysicalBattery {
    pub async fn new(
        connection: &Connection,
        path: OwnedObjectPath,
        cancellation_token: CancellationToken,
    ) -> Result<Self> {
        let device_proxy = DeviceProxy::builder(&connection)
            .path(path.clone())?
            .build()
            .await?;

        let native_path = Property::new(device_proxy.native_path().await?);
        let vendor = Property::new(device_proxy.vendor().await?);
        let model = Property::new(device_proxy.model().await?);
        let serial = Property::new(device_proxy.serial().await?);
        let kind = Property::new(Kind::from(device_proxy.kind().await?));
        let time_to_empty = Property::new(Duration::from_secs(
            device_proxy.time_to_empty().await? as u64,
        ));
        let time_to_full = Property::new(Duration::from_secs(
            device_proxy.time_to_full().await? as u64,
        ));
        let percentage = Property::new(device_proxy.percentage().await?);
        let temperature = Property::new(device_proxy.temperature().await?);
        let state = Property::new(State::from(device_proxy.state().await?));
        let is_rechargeable = Property::new(device_proxy.is_rechargeable().await?);
        let capacity = Property::new(device_proxy.capacity().await?);
        let charge_start_threshold = Property::new(device_proxy.charge_start_threshold().await?);
        let charge_end_threshold = Property::new(device_proxy.charge_end_threshold().await?);
        let charge_threshold_enabled =
            Property::new(device_proxy.charge_threshold_enabled().await?);
        let charge_threshold_supported =
            Property::new(device_proxy.charge_threshold_supported().await?);
        let charge_threshold_settings_supported =
            Property::new(device_proxy.charge_threshold_settings_supported().await?);

        {
            let cancellation_token = cancellation_token.clone();

            let time_to_empty = time_to_empty.clone();
            let time_to_full = time_to_full.clone();
            let percentage = percentage.clone();
            let temperature = temperature.clone();
            let state = state.clone();
            let is_rechargeable = is_rechargeable.clone();
            let capacity = capacity.clone();
            let charge_start_threshold = charge_start_threshold.clone();
            let charge_end_threshold = charge_end_threshold.clone();
            let charge_threshold_enabled = charge_threshold_enabled.clone();
            let charge_threshold_supported = charge_threshold_supported.clone();
            let charge_threshold_settings_supported = charge_threshold_settings_supported.clone();

            let mut time_to_empty_stream = device_proxy.receive_time_to_empty_changed().await;
            let mut time_to_full_stream = device_proxy.receive_time_to_full_changed().await;
            let mut percentage_stream = device_proxy.receive_percentage_changed().await;
            let mut temperature_stream = device_proxy.receive_temperature_changed().await;
            let mut state_stream = device_proxy.receive_state_changed().await;
            let mut is_rechargeable_stream = device_proxy.receive_is_rechargeable_changed().await;
            let mut capacity_stream = device_proxy.receive_capacity_changed().await;
            let mut charge_start_threshold_stream =
                device_proxy.receive_charge_start_threshold_changed().await;
            let mut charge_end_threshold_stream =
                device_proxy.receive_charge_end_threshold_changed().await;
            let mut charge_threshold_enabled_stream = device_proxy
                .receive_charge_threshold_enabled_changed()
                .await;
            let mut charge_threshold_supported_stream = device_proxy
                .receive_charge_threshold_supported_changed()
                .await;
            let mut charge_threshold_settings_supported_stream = device_proxy
                .receive_charge_threshold_settings_supported_changed()
                .await;

            relm4::spawn(async move {
                loop {
                    async_select::select! {
                        Some(change) = time_to_empty_stream.next() => {
                            if let Ok(value) = change.get().await {
                                let value = Duration::from_secs(value as u64);
                                time_to_empty.write(value);
                            }
                        }
                        Some(change) = time_to_full_stream.next() => {
                            if let Ok(value) = change.get().await {
                                let value = Duration::from_secs(value as u64);
                                time_to_full.write(value);
                            }
                        }
                        Some(change) = percentage_stream.next() => {
                            if let Ok(value) = change.get().await {
                                percentage.write(value);
                            }
                        }
                        Some(change) = temperature_stream.next() => {
                            if let Ok(value) = change.get().await {
                                temperature.write(value);
                            }
                        }
                        Some(change) = state_stream.next() => {
                            if let Ok(value) = change.get().await {
                                state.write(State::from(value));
                            }
                        }
                        Some(change) = is_rechargeable_stream.next() => {
                            if let Ok(value) = change.get().await {
                                is_rechargeable.write(value);
                            }
                        }
                        Some(change) = capacity_stream.next() => {
                            if let Ok(value) = change.get().await {
                                capacity.write(value);
                            }
                        }
                        Some(change) = charge_start_threshold_stream.next() => {
                            if let Ok(value) = change.get().await {
                                charge_start_threshold.write(value);
                            }
                        }
                        Some(change) = charge_end_threshold_stream.next() => {
                            if let Ok(value) = change.get().await {
                                charge_end_threshold.write(value);
                            }
                        }
                        Some(change) = charge_threshold_enabled_stream.next() => {
                            if let Ok(value) = change.get().await {
                                charge_threshold_enabled.write(value);
                            }
                        }
                        Some(change) = charge_threshold_supported_stream.next() => {
                            if let Ok(value) = change.get().await {
                                charge_threshold_supported.write(value);
                            }
                        }
                        Some(change) = charge_threshold_settings_supported_stream.next() => {
                            if let Ok(value) = change.get().await {
                                charge_threshold_settings_supported.write(value);
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
            path,
            cancellation_token,
            device_proxy,

            native_path,
            vendor,
            model,
            serial,
            kind,
            time_to_empty,
            time_to_full,
            percentage,
            temperature,
            state,
            is_rechargeable,
            capacity,
            charge_start_threshold,
            charge_end_threshold,
            charge_threshold_enabled,
            charge_threshold_supported,
            charge_threshold_settings_supported,
        })
    }

    pub async fn enable_charge_threshold(&self, enabled: bool) -> Result<()> {
        self.device_proxy.enable_charge_threshold(enabled).await?;
        Ok(())
    }
}

impl Drop for PhysicalBattery {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
