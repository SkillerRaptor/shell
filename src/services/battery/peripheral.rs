//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

use anyhow::Result;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::{
    core::property::Property,
    services::battery::{proxy::DeviceProxy, types::Kind},
};

pub struct Peripheral {
    pub(super) path: OwnedObjectPath,
    device_proxy: DeviceProxy<'static>,

    pub native_path: Property<String>,
    pub vendor: Property<String>,
    pub model: Property<String>,
    pub serial: Property<String>,
    pub kind: Property<Kind>,
}

impl Peripheral {
    pub async fn new(connection: &Connection, path: OwnedObjectPath) -> Result<Self> {
        let device_proxy = DeviceProxy::builder(&connection)
            .path(path.clone())?
            .build()
            .await?;

        let native_path = Property::new(device_proxy.native_path().await?);
        let vendor = Property::new(device_proxy.vendor().await?);
        let model = Property::new(device_proxy.model().await?);
        let serial = Property::new(device_proxy.serial().await?);
        let kind = Property::new(device_proxy.kind().await?);

        Ok(Self {
            path,
            device_proxy,

            native_path,
            vendor,
            model,
            serial,
            kind,
        })
    }
}
