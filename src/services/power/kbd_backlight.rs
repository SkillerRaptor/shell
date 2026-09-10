//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

use anyhow::Result;
use futures_util::StreamExt;
use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::{core::property::Property, services::power::proxy::KbdBacklightProxy};

pub struct KbdBacklight {
    kbd_backlight_proxy: KbdBacklightProxy<'static>,

    pub path: OwnedObjectPath,

    pub max_brightness: Property<i32>,
    pub brightness: Property<i32>,
    pub native_path: Property<String>,
}

impl KbdBacklight {
    pub async fn new(
        connection: &Connection,
        path: OwnedObjectPath,
        cancellation_token: CancellationToken,
    ) -> Result<Self> {
        let kbd_backlight_proxy = KbdBacklightProxy::builder(&connection)
            .path(path.clone())?
            .build()
            .await?;

        let max_brightness = Property::new(kbd_backlight_proxy.get_max_brightness().await?);
        let brightness = Property::new(kbd_backlight_proxy.get_brightness().await?);
        let native_path = Property::new(kbd_backlight_proxy.native_path().await?);

        {
            let brightness = brightness.clone();
            let native_path = native_path.clone();

            let mut brightness_stream = kbd_backlight_proxy.receive_brightness_changed().await?;
            let mut native_path_stream = kbd_backlight_proxy.receive_native_path_changed().await;

            relm4::spawn(async move {
                loop {
                    async_select::select! {
                        Some(change) = brightness_stream.next() => {
                            if let Ok(args) = change.args() {
                                brightness.write(args.value);
                            }
                        }
                        Some(change) = native_path_stream.next() => {
                            if let Ok(value) = change.get().await {
                                native_path.write(value);
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
            kbd_backlight_proxy,

            max_brightness,
            brightness,
            native_path,
        })
    }

    async fn set_brightness(&self, value: i32) -> Result<()> {
        self.kbd_backlight_proxy.set_brightness(value).await?;
        Ok(())
    }
}
