use derive_more::Display;
use serde::{Deserialize, Serialize};

pub static AGE_ATTESTATION_INTERFACE: &str = "org.bigbother.AgeAttestation1";
pub static AGE_ATTESTATION_OBJECT_PATH: &str = "/org/bigbother/AgeAttestation1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum AgeBracket {
    #[display("under_13")]
    Child,
    #[display("13_to_15")]
    YoungTeen,
    #[display("16_to_17")]
    OlderTeen,
    #[display("18_plus")]
    Adult,
}

impl AgeBracket {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Child => "under 13",
            Self::YoungTeen => "13-15",
            Self::OlderTeen => "16-17",
            Self::Adult => "18+",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Child => "under_13",
            Self::YoungTeen => "13_to_15",
            Self::OlderTeen => "16_to_17",
            Self::Adult => "18_plus",
        }
    }
}

impl From<u8> for AgeBracket {
    fn from(age: u8) -> Self {
        match age {
            0..=12 => Self::Child,
            13..=15 => Self::YoungTeen,
            16..=17 => Self::OlderTeen,
            _ => Self::Adult,
        }
    }
}

impl TryFrom<&str> for AgeBracket {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "under_13" => Ok(Self::Child),
            "13_to_15" => Ok(Self::YoungTeen),
            "16_to_17" => Ok(Self::OlderTeen),
            "18_plus" => Ok(Self::Adult),
            _ => Err(format!("unknown age bracket: {s}")),
        }
    }
}
