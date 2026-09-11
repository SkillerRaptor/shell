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
        types::{BatteryLevel, State, Type},
    },
};

pub struct Battery {
    pub(super) path: OwnedObjectPath,
    cancellation_token: CancellationToken,
    device_proxy: DeviceProxy<'static>,

    pub native_path: Property<String>,
    pub vendor: Property<String>,
    pub model: Property<String>,
    pub serial: Property<String>,
    pub update_time: Property<u64>,
    pub r#type: Property<Type>,
    pub power_supply: Property<bool>,
    pub online: Property<bool>,
    pub energy: Property<f64>,
    pub energy_empty: Property<f64>,
    pub energy_full: Property<f64>,
    pub energy_full_design: Property<f64>,
    pub energy_rate: Property<f64>,
    pub charge_cycles: Property<i32>,
    pub time_to_empty: Property<Duration>,
    pub time_to_full: Property<Duration>,
    pub percentage: Property<f64>,
    pub temperature: Property<f64>,
    pub is_present: Property<bool>,
    pub state: Property<State>,
    pub is_rechargeable: Property<bool>,
    pub capacity: Property<f64>,
    pub battery_level: Property<BatteryLevel>,
    pub charge_start_threshold: Property<u32>,
    pub charge_end_threshold: Property<u32>,
    pub charge_threshold_enabled: Property<bool>,
    pub charge_threshold_supported: Property<bool>,
    pub charge_threshold_settings_supported: Property<u32>,
}

impl Battery {
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
        let update_time = Property::new(device_proxy.update_time().await?);
        let r#type = Property::new(Type::from(device_proxy.r#type().await?));
        let power_supply = Property::new(device_proxy.power_supply().await?);
        let online = Property::new(device_proxy.online().await?);
        let energy = Property::new(device_proxy.energy().await?);
        let energy_empty = Property::new(device_proxy.energy_empty().await?);
        let energy_full = Property::new(device_proxy.energy_full().await?);
        let energy_full_design = Property::new(device_proxy.energy_full_design().await?);
        let energy_rate = Property::new(device_proxy.energy_rate().await?);
        let charge_cycles = Property::new(device_proxy.charge_cycles().await?);
        let time_to_empty = Property::new(Duration::from_secs(
            device_proxy.time_to_empty().await? as u64,
        ));
        let time_to_full = Property::new(Duration::from_secs(
            device_proxy.time_to_full().await? as u64,
        ));
        let percentage = Property::new(device_proxy.percentage().await?);
        let temperature = Property::new(device_proxy.temperature().await?);
        let is_present = Property::new(device_proxy.is_present().await?);
        let state = Property::new(State::from(device_proxy.state().await?));
        let is_rechargeable = Property::new(device_proxy.is_rechargeable().await?);
        let capacity = Property::new(device_proxy.capacity().await?);
        let battery_level = Property::new(BatteryLevel::from(device_proxy.battery_level().await?));
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

            let native_path = native_path.clone();
            let vendor = vendor.clone();
            let model = model.clone();
            let serial = serial.clone();
            let update_time = update_time.clone();
            let r#type = r#type.clone();
            let power_supply = power_supply.clone();
            let online = online.clone();
            let energy = energy.clone();
            let energy_empty = energy_empty.clone();
            let energy_full = energy_full.clone();
            let energy_full_design = energy_full_design.clone();
            let energy_rate = energy_rate.clone();
            let charge_cycles = charge_cycles.clone();
            let time_to_empty = time_to_empty.clone();
            let time_to_full = time_to_full.clone();
            let percentage = percentage.clone();
            let temperature = temperature.clone();
            let is_present = is_present.clone();
            let state = state.clone();
            let is_rechargeable = is_rechargeable.clone();
            let capacity = capacity.clone();
            let battery_level = battery_level.clone();
            let charge_start_threshold = charge_start_threshold.clone();
            let charge_end_threshold = charge_end_threshold.clone();
            let charge_threshold_enabled = charge_threshold_enabled.clone();
            let charge_threshold_supported = charge_threshold_supported.clone();
            let charge_threshold_settings_supported = charge_threshold_settings_supported.clone();

            let mut native_path_stream = device_proxy.receive_native_path_changed().await;
            let mut vendor_stream = device_proxy.receive_vendor_changed().await;
            let mut model_stream = device_proxy.receive_model_changed().await;
            let mut serial_stream = device_proxy.receive_serial_changed().await;
            let mut update_time_stream = device_proxy.receive_update_time_changed().await;
            let mut type_stream = device_proxy.receive_type_changed().await;
            let mut power_supply_stream = device_proxy.receive_power_supply_changed().await;
            let mut online_stream = device_proxy.receive_online_changed().await;
            let mut energy_stream = device_proxy.receive_energy_changed().await;
            let mut energy_empty_stream = device_proxy.receive_energy_empty_changed().await;
            let mut energy_full_stream = device_proxy.receive_energy_full_changed().await;
            let mut energy_full_design_stream =
                device_proxy.receive_energy_full_design_changed().await;
            let mut energy_rate_stream = device_proxy.receive_energy_rate_changed().await;
            let mut charge_cycles_stream = device_proxy.receive_charge_cycles_changed().await;
            let mut time_to_empty_stream = device_proxy.receive_time_to_empty_changed().await;
            let mut time_to_full_stream = device_proxy.receive_time_to_full_changed().await;
            let mut percentage_stream = device_proxy.receive_percentage_changed().await;
            let mut temperature_stream = device_proxy.receive_temperature_changed().await;
            let mut is_present_stream = device_proxy.receive_is_present_changed().await;
            let mut state_stream = device_proxy.receive_state_changed().await;
            let mut is_rechargeable_stream = device_proxy.receive_is_rechargeable_changed().await;
            let mut capacity_stream = device_proxy.receive_capacity_changed().await;
            let mut battery_level_stream = device_proxy.receive_battery_level_changed().await;
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
                        Some(change) = native_path_stream.next() => {
                            if let Ok(value) = change.get().await {
                                native_path.write(value);
                            }
                        }
                        Some(change) = vendor_stream.next() => {
                            if let Ok(value) = change.get().await {
                                vendor.write(value);
                            }
                        }
                        Some(change) = model_stream.next() => {
                            if let Ok(value) = change.get().await {
                                model.write(value);
                            }
                        }
                        Some(change) = serial_stream.next() => {
                            if let Ok(value) = change.get().await {
                                serial.write(value);
                            }
                        }
                        Some(change) = update_time_stream.next() => {
                            if let Ok(value) = change.get().await {
                                update_time.write(value);
                            }
                        }
                        Some(change) = type_stream.next() => {
                            if let Ok(value) = change.get().await {
                                r#type.write(Type::from(value));
                            }
                        }
                        Some(change) = power_supply_stream.next() => {
                            if let Ok(value) = change.get().await {
                                power_supply.write(value);
                            }
                        }
                        Some(change) = online_stream.next() => {
                            if let Ok(value) = change.get().await {
                                online.write(value);
                            }
                        }
                        Some(change) = energy_stream.next() => {
                            if let Ok(value) = change.get().await {
                                energy.write(value);
                            }
                        }
                        Some(change) = energy_empty_stream.next() => {
                            if let Ok(value) = change.get().await {
                                energy_empty.write(value);
                            }
                        }
                        Some(change) = energy_full_stream.next() => {
                            if let Ok(value) = change.get().await {
                                energy_full.write(value);
                            }
                        }
                        Some(change) = energy_full_design_stream.next() => {
                            if let Ok(value) = change.get().await {
                                energy_full_design.write(value);
                            }
                        }
                        Some(change) = energy_rate_stream.next() => {
                            if let Ok(value) = change.get().await {
                                energy_rate.write(value);
                            }
                        }
                        Some(change) = charge_cycles_stream.next() => {
                            if let Ok(value) = change.get().await {
                                charge_cycles.write(value);
                            }
                        }
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
                        Some(change) = is_present_stream.next() => {
                            if let Ok(value) = change.get().await {
                                is_present.write(value);
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
                        Some(change) = battery_level_stream.next() => {
                            if let Ok(value) = change.get().await {
                                battery_level.write(BatteryLevel::from(value));
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
            update_time,
            r#type,
            power_supply,
            online,
            energy,
            energy_empty,
            energy_full,
            energy_full_design,
            energy_rate,
            charge_cycles,
            time_to_empty,
            time_to_full,
            percentage,
            temperature,
            is_present,
            state,
            is_rechargeable,
            capacity,
            battery_level,
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

impl Drop for Battery {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
