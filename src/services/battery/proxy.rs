//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

use zbus::zvariant::{ObjectPath, OwnedObjectPath};

#[zbus::proxy(
    interface = "org.freedesktop.UPower",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower"
)]
pub trait UPower {
    fn enumerate_devices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    fn get_display_device(&self) -> zbus::Result<OwnedObjectPath>;

    #[zbus(signal)]
    fn device_added(&self, device: ObjectPath<'_>) -> zbus::Result<()>;

    #[zbus(signal)]
    fn device_removed(&self, device: ObjectPath<'_>) -> zbus::Result<()>;

    #[zbus(property)]
    fn on_battery(&self) -> zbus::Result<bool>;
}

#[zbus::proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower"
)]
pub trait Device {
    fn enable_charge_threshold(&self, enabled: bool) -> zbus::Result<()>;

    #[zbus(property)]
    fn native_path(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn vendor(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn model(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn serial(&self) -> zbus::Result<String>;

    #[zbus(property, name = "Type")]
    fn kind(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn time_to_empty(&self) -> zbus::Result<i64>;

    #[zbus(property)]
    fn time_to_full(&self) -> zbus::Result<i64>;

    #[zbus(property)]
    fn percentage(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn temperature(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn is_rechargeable(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn capacity(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn charge_start_threshold(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn charge_end_threshold(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn charge_threshold_enabled(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn charge_threshold_supported(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn charge_threshold_settings_supported(&self) -> zbus::Result<u32>;
}
