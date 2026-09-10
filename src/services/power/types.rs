//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
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

impl From<u32> for Type {
    fn from(value: u32) -> Self {
        match value {
            0 => Type::Unknown,
            1 => Type::LinePower,
            2 => Type::Battery,
            3 => Type::Ups,
            4 => Type::Monitor,
            5 => Type::Mouse,
            6 => Type::Keyboard,
            7 => Type::Pda,
            8 => Type::Phone,
            9 => Type::MediaPlayer,
            10 => Type::Tablet,
            11 => Type::Computer,
            12 => Type::GamingInput,
            13 => Type::Pen,
            14 => Type::Touchpad,
            15 => Type::Modem,
            16 => Type::Network,
            17 => Type::Headset,
            18 => Type::Speakers,
            19 => Type::Headphones,
            20 => Type::Video,
            21 => Type::OtherAudio,
            22 => Type::RemoteControl,
            23 => Type::Printer,
            24 => Type::Scanner,
            25 => Type::Camera,
            26 => Type::Wearable,
            27 => Type::Toy,
            28 => Type::BluetoothGeneric,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Technology {
    Unknown,
    LithiumIon,
    LithiumPolymer,
    LithiumIronPhosphate,
    LeadAcid,
    NickelCadmium,
    NickelMetalHydride,
}

impl From<u32> for Technology {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::LithiumIon,
            2 => Self::LithiumPolymer,
            3 => Self::LithiumIronPhosphate,
            4 => Self::LeadAcid,
            5 => Self::NickelCadmium,
            6 => Self::NickelMetalHydride,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarningLevel {
    Unknown,
    None,
    Discharging,
    Low,
    Critical,
    Action,
}

impl From<u32> for WarningLevel {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::None,
            2 => Self::Discharging,
            3 => Self::Low,
            4 => Self::Critical,
            5 => Self::Action,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatteryLevel {
    Unknown,
    None,
    Low,
    Critical,
    Normal,
    High,
    Full,
}

impl From<u32> for BatteryLevel {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::None,
            2 => Self::Low,
            3 => Self::Critical,
            4 => Self::Normal,
            5 => Self::High,
            6 => Self::Full,
            _ => unreachable!(),
        }
    }
}
