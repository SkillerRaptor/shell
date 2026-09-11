//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

pub mod cpu;
pub mod disk;
pub mod memory;
pub mod network;

use anyhow::Result;
use tokio_util::sync::CancellationToken;

use crate::{
    core::property::Property,
    services::system_info::{
        cpu::CpuData,
        disk::DiskData,
        memory::MemoryData,
        network::NetworkData,
    },
};

pub struct SystemInfoService {
    cancellation_token: CancellationToken,

    pub cpu: Property<CpuData>,
    pub memory: Property<MemoryData>,
    pub disks: Property<Vec<DiskData>>,
    pub networks: Property<Vec<NetworkData>>,
}

impl SystemInfoService {
    pub async fn new() -> Result<Self> {
        let cancellation_token = CancellationToken::new();

        let cpu = Property::new(CpuData::default());
        cpu::spawn_cpu_proxy(cancellation_token.child_token(), cpu.clone());

        let memory = Property::new(MemoryData::default());
        memory::spawn_memory_proxy(cancellation_token.child_token(), memory.clone());

        let disks = Property::new(Vec::default());
        disk::spawn_disk_proxy(cancellation_token.child_token(), disks.clone());

        let networks = Property::new(Vec::default());
        network::spawn_network_proxy(cancellation_token.child_token(), networks.clone());

        Ok(Self {
            cancellation_token,

            cpu,
            memory,
            disks,
            networks,
        })
    }
}

impl Drop for SystemInfoService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
