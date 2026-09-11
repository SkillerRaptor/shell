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
        types::{State, Type},
    },
};

pub struct DisplayDevice {
    device_proxy: DeviceProxy<'static>,

    pub r#type: Property<Type>,
    pub state: Property<State>,
    pub percentage: Property<f64>,
    pub energy: Property<f64>,
    pub energy_full: Property<f64>,
    pub energy_rate: Property<f64>,
    pub time_to_empty: Property<Duration>,
    pub time_to_full: Property<Duration>,
    pub is_present: Property<bool>,
}

impl DisplayDevice {
    pub async fn new(
        connection: &Connection,
        path: OwnedObjectPath,
        cancellation_token: CancellationToken,
    ) -> Result<Self> {
        let device_proxy = DeviceProxy::builder(&connection)
            .path(path.clone())?
            .build()
            .await?;

        let r#type = Property::new(Type::from(device_proxy.r#type().await?));
        let state = Property::new(State::from(device_proxy.state().await?));
        let percentage = Property::new(device_proxy.percentage().await?);
        let energy = Property::new(device_proxy.energy().await?);
        let energy_full = Property::new(device_proxy.energy_full().await?);
        let energy_rate = Property::new(device_proxy.energy_rate().await?);
        let time_to_empty = Property::new(Duration::from_secs(
            device_proxy.time_to_empty().await? as u64,
        ));
        let time_to_full = Property::new(Duration::from_secs(
            device_proxy.time_to_full().await? as u64,
        ));
        let is_present = Property::new(device_proxy.is_present().await?);

        {
            let r#type = r#type.clone();
            let state = state.clone();
            let percentage = percentage.clone();
            let energy = energy.clone();
            let energy_full = energy_full.clone();
            let energy_rate = energy_rate.clone();
            let time_to_empty = time_to_empty.clone();
            let time_to_full = time_to_full.clone();
            let is_present = is_present.clone();

            let mut type_stream = device_proxy.receive_type_changed().await;
            let mut state_stream = device_proxy.receive_state_changed().await;
            let mut percentage_stream = device_proxy.receive_percentage_changed().await;
            let mut energy_stream = device_proxy.receive_energy_changed().await;
            let mut energy_full_stream = device_proxy.receive_energy_full_changed().await;
            let mut energy_rate_stream = device_proxy.receive_energy_rate_changed().await;
            let mut time_to_empty_stream = device_proxy.receive_time_to_empty_changed().await;
            let mut time_to_full_stream = device_proxy.receive_time_to_full_changed().await;
            let mut is_present_stream = device_proxy.receive_is_present_changed().await;

            relm4::spawn(async move {
                loop {
                    async_select::select! {
                        Some(change) = type_stream.next() => {
                            if let Ok(value) = change.get().await {
                                r#type.write(Type::from(value));
                            }
                        }
                        Some(change) = state_stream.next() => {
                            if let Ok(value) = change.get().await {
                                state.write(State::from(value));
                            }
                        }
                        Some(change) = percentage_stream.next() => {
                            if let Ok(value) = change.get().await {
                                percentage.write(value);
                            }
                        }
                        Some(change) = energy_stream.next() => {
                            if let Ok(value) = change.get().await {
                                energy.write(value);
                            }
                        }
                        Some(change) = energy_full_stream.next() => {
                            if let Ok(value) = change.get().await {
                                energy_full.write(value);
                            }
                        }
                        Some(change) = energy_rate_stream.next() => {
                            if let Ok(value) = change.get().await {
                                energy_rate.write(value);
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
                        Some(change) = is_present_stream.next() => {
                            if let Ok(value) = change.get().await {
                                is_present.write(value);
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
            device_proxy,

            r#type,
            state,
            percentage,
            energy,
            energy_full,
            energy_rate,
            time_to_empty,
            time_to_full,
            is_present,
        })
    }
}
