//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Unknown,
    LinePower,
    Battery,
    Ups,
    Monitor,
    Mouse,
    Keyboard,
    Pda,
    Phone,
    MediaPlayer,
    Tablet,
    Computer,
    GamingInput,
    Pen,
    Touchpad,
    Modem,
    Network,
    Headset,
    Speakers,
    Headphones,
    Video,
    OtherAudio,
    RemoteControl,
    Printer,
    Scanner,
    Camera,
    Wearable,
    Toy,
    BluetoothGeneric,
}

impl From<u32> for Kind {
    fn from(value: u32) -> Self {
        match value {
            0 => Kind::Unknown,
            1 => Kind::LinePower,
            2 => Kind::Battery,
            3 => Kind::Ups,
            4 => Kind::Monitor,
            5 => Kind::Mouse,
            6 => Kind::Keyboard,
            7 => Kind::Pda,
            8 => Kind::Phone,
            9 => Kind::MediaPlayer,
            10 => Kind::Tablet,
            11 => Kind::Computer,
            12 => Kind::GamingInput,
            13 => Kind::Pen,
            14 => Kind::Touchpad,
            15 => Kind::Modem,
            16 => Kind::Network,
            17 => Kind::Headset,
            18 => Kind::Speakers,
            19 => Kind::Headphones,
            20 => Kind::Video,
            21 => Kind::OtherAudio,
            22 => Kind::RemoteControl,
            23 => Kind::Printer,
            24 => Kind::Scanner,
            25 => Kind::Camera,
            26 => Kind::Wearable,
            27 => Kind::Toy,
            28 => Kind::BluetoothGeneric,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Unknown,
    Charging,
    Discharging,
    Empty,
    FullyCharged,
    PendingCharge,
    PendingDischarge,
}

impl From<u32> for State {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Charging,
            2 => Self::Discharging,
            3 => Self::Empty,
            4 => Self::FullyCharged,
            5 => Self::PendingCharge,
            6 => Self::PendingDischarge,
            _ => unreachable!(),
        }
    }
}
