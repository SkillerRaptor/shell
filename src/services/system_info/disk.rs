//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

use std::time::Duration;

use sysinfo::Disks;
use tokio_util::sync::CancellationToken;

use crate::core::property::Property;

#[derive(Clone, Debug, Default)]
pub struct DiskData {
    pub name: String,
    pub file_system: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_bytes: u64,
    pub usage: f32,
    pub is_removable: bool,
    pub is_read_only: bool,
}

pub fn spawn_disk_proxy(cancellation_token: CancellationToken, disks: Property<Vec<DiskData>>) {
    relm4::spawn(async move {
        let mut sysinfo_disks = Disks::new_with_refreshed_list();
        let mut interval = tokio::time::interval(Duration::from_secs(15));

        loop {
            sysinfo_disks.refresh(false);

            let data = sysinfo_disks
                .iter()
                .map(|disk| {
                    let total_bytes = disk.total_space();
                    let available_bytes = disk.available_space();
                    let used_bytes = total_bytes.saturating_sub(available_bytes);

                    let usage = if total_bytes > 0 {
                        (used_bytes as f32 / total_bytes as f32) * 100.0
                    } else {
                        0.0
                    };

                    DiskData {
                        name: disk.name().to_string_lossy().to_string(),
                        file_system: disk.file_system().to_string_lossy().to_string(),
                        mount_point: disk.mount_point().to_string_lossy().to_string(),
                        total_bytes,
                        available_bytes,
                        used_bytes,
                        usage,
                        is_removable: disk.is_removable(),
                        is_read_only: disk.is_read_only(),
                    }
                })
                .collect::<Vec<_>>();

            disks.write_unconditional(data);

            async_select::select! {
                _ = interval.tick() => {}
                _ = cancellation_token.cancelled() => {
                    break;
                }
            }
        }
    });
}
