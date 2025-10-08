use crate::model::free::ServerCheck;
use serde::{Deserialize, Serialize};

pub enum CelebClub {
    /// 云从社
    Ycs,
    /// 楚天社
    Cts,
    /// 披风会
    Pfh,
}

impl CelebClub {
    pub fn from_str(s: &str) -> Option<CelebClub> {
        match s {
            "云从社" => Some(CelebClub::Ycs),
            "楚天社" => Some(CelebClub::Cts),
            "披风会" => Some(CelebClub::Pfh),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            CelebClub::Ycs => "云从社",
            CelebClub::Cts => "楚天社",
            CelebClub::Pfh => "披风会",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ServerCheckEnums {
    Object(ServerCheck),
    Array(Vec<ServerCheck>),
}
