//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

use std::time::Duration;

use sysinfo::System;
use tokio_util::sync::CancellationToken;

use crate::core::property::Property;

#[derive(Clone, Debug, Default)]
pub struct MemoryData {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage: f32,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_usage: f32,
}

pub fn spawn_memory_proxy(cancellation_token: CancellationToken, memory: Property<MemoryData>) {
    relm4::spawn(async move {
        let mut system = System::new();
        let mut interval = tokio::time::interval(Duration::from_secs(2));

        loop {
            system.refresh_memory();

            let total_bytes = system.total_memory();
            let available_bytes = system.available_memory();
            let used_bytes = system.used_memory();

            let usage = if total_bytes > 0 {
                (used_bytes as f32 / total_bytes as f32) * 100.0
            } else {
                0.0
            };

            let swap_total_bytes = system.total_swap();
            let swap_used_bytes = system.used_swap();

            let swap_usage = if swap_total_bytes > 0 {
                (swap_used_bytes as f32 / swap_total_bytes as f32) * 100.0
            } else {
                0.0
            };

            memory.write_unconditional(MemoryData {
                total_bytes,
                used_bytes,
                available_bytes,
                usage,
                swap_total_bytes,
                swap_used_bytes,
                swap_usage,
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
