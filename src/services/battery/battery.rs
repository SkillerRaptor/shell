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

pub struct Battery {
    cancellation_token: CancellationToken,
    device_proxy: DeviceProxy<'static>,

    pub kind: Property<Kind>,
    pub state: Property<State>,
    pub percentage: Property<f64>,
    pub time_to_empty: Property<Duration>,
    pub time_to_full: Property<Duration>,
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

        let kind = Property::new(device_proxy.kind().await?);
        let state = Property::new(device_proxy.state().await?);
        let percentage = Property::new(device_proxy.percentage().await?);
        let time_to_empty = Property::new(Duration::from_secs(
            device_proxy.time_to_empty().await? as u64,
        ));
        let time_to_full = Property::new(Duration::from_secs(
            device_proxy.time_to_full().await? as u64,
        ));

        {
            let cancellation_token = cancellation_token.clone();

            let state = state.clone();
            let percentage = percentage.clone();
            let time_to_empty = time_to_empty.clone();
            let time_to_full = time_to_full.clone();

            let mut state_stream = device_proxy.receive_state_changed().await;
            let mut percentage_stream = device_proxy.receive_percentage_changed().await;
            let mut time_to_empty_stream = device_proxy.receive_time_to_empty_changed().await;
            let mut time_to_full_stream = device_proxy.receive_time_to_full_changed().await;

            relm4::spawn(async move {
                loop {
                    async_select::select! {
                        Some(change) = state_stream.next() => {
                            if let Ok(value) = change.get().await {
                                state.write(value);
                            }
                        }
                        Some(change) = percentage_stream.next() => {
                            if let Ok(value) = change.get().await {
                                percentage.write(value);
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
                        _ = cancellation_token.cancelled() => {
                            break;
                        }
                    }
                }
            });
        }

        Ok(Self {
            cancellation_token,
            device_proxy,

            kind,
            state,
            percentage,
            time_to_empty,
            time_to_full,
        })
    }
}

impl Drop for Battery {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
