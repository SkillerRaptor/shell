//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

use std::{collections::HashMap, time::Duration};

use sysinfo::Networks;
use tokio_util::sync::CancellationToken;

use crate::core::property::Property;

#[derive(Clone, Debug, Default)]
pub struct NetworkData {
    pub interface: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_bytes_per_second: u64,
    pub tx_bytes_per_second: u64,
}

pub fn spawn_network_proxy(
    cancellation_token: CancellationToken,
    networks: Property<Vec<NetworkData>>,
) {
    relm4::spawn(async move {
        let mut sysinfo_networks = Networks::new_with_refreshed_list();
        let mut interval = tokio::time::interval(Duration::from_secs(2));
        let mut previous_rx = HashMap::new();
        let mut previous_tx = HashMap::new();

        loop {
            sysinfo_networks.refresh(true);

            let data = sysinfo_networks
                .iter()
                .map(|(name, network)| {
                    let rx_bytes = network.total_received();
                    let tx_bytes = network.total_transmitted();

                    let last_rx = previous_rx.get(name).copied().unwrap_or(rx_bytes);
                    let last_tx = previous_tx.get(name).copied().unwrap_or(tx_bytes);

                    let rx_delta = rx_bytes.saturating_sub(last_rx);
                    let tx_delta = tx_bytes.saturating_sub(last_tx);

                    let rx_bytes_per_second =
                        (rx_delta as f64 / interval.period().as_secs() as f64) as u64;
                    let tx_bytes_per_second =
                        (tx_delta as f64 / interval.period().as_secs() as f64) as u64;

                    previous_rx.insert(name.clone(), rx_bytes);
                    previous_tx.insert(name.clone(), tx_bytes);

                    NetworkData {
                        interface: name.clone(),
                        rx_bytes,
                        tx_bytes,
                        rx_bytes_per_second,
                        tx_bytes_per_second,
                    }
                })
                .collect::<Vec<_>>();

            networks.write_unconditional(data);

            async_select::select! {
                _ = interval.tick() => {}
                _ = cancellation_token.cancelled() => {
                    break;
                }
            }
        }
    });
}
