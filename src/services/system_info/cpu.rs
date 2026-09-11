//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

use std::time::Duration;

use sysinfo::{Components, System};
use tokio_util::sync::CancellationToken;

use crate::core::property::Property;

#[derive(Clone, Debug, Default)]
pub struct CoreData {
    pub name: String,
    pub usage: f32,
    pub frequency: u64,
}

#[derive(Clone, Debug, Default)]
pub struct CpuData {
    pub vendor_id: String,
    pub brand: String,
    pub usage: f32,
    pub average_frequency: u64,
    pub max_frequency: u64,
    pub temperature: Option<f32>,
    pub cores: Vec<CoreData>,
}

pub fn spawn_cpu_proxy(cancellation_token: CancellationToken, cpu: Property<CpuData>) {
    relm4::spawn(async move {
        let mut system = System::new();
        let mut components = Components::new_with_refreshed_list();
        let mut interval = tokio::time::interval(Duration::from_secs(2));

        loop {
            system.refresh_cpu_all();
            components.refresh(false);

            let cores = system
                .cpus()
                .iter()
                .map(|core| CoreData {
                    name: core.name().to_owned(),
                    usage: core.cpu_usage(),
                    frequency: core.frequency(),
                })
                .collect::<Vec<_>>();

            let (average_frequency, max_frequency) = if cores.is_empty() {
                (0, 0)
            } else {
                let average =
                    (cores.iter().map(|core| core.frequency).sum::<u64>()) / cores.len() as u64;
                let max = cores
                    .iter()
                    .map(|core| core.frequency)
                    .max()
                    .unwrap_or_default();
                (average, max)
            };

            const CPU_TEMPERATURE_PATTERNS: &[&str] = &[
                "tctl",
                "tdie",
                "tccd",
                "k10temp",
                "coretemp",
                "package id",
                "cpu",
            ];

            let temperature = CPU_TEMPERATURE_PATTERNS.iter().find_map(|pattern| {
                components
                    .iter()
                    .find(|component| component.label().to_lowercase().contains(pattern))
                    .and_then(|component| component.temperature())
            });

            cpu.write_unconditional(CpuData {
                brand: system.cpus()[0].brand().to_owned(),
                vendor_id: system.cpus()[0].vendor_id().to_owned(),
                usage: system.global_cpu_usage(),
                average_frequency,
                max_frequency,
                temperature,
                cores,
            });

            async_select::select! {
                _ = interval.tick() => {}
                _ = cancellation_token.cancelled() => {
                    break;
                }
            }
        }
    });
}
