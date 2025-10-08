use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Calendar {
    pub date: String,
    pub week: String,
    pub war: String,
    pub battle: String,
    pub orecar: String,
    pub school: String,
    pub rescue: String,
    pub luck: Option<Vec<String>>,
    pub card: Vec<String>,
    pub draw: Option<String>,
    pub team: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Today {
    pub date: String,
    pub week: String,
    pub year: String,
    pub month: String,
    pub day: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListCalendar {
    pub today: Option<Today>,
    pub data: Option<Vec<Calendar>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Celeb {
    pub map: String,
    pub stage: String,
    pub site: String,
    pub desc: String,
    pub icon: String,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub correctness: i32,
    pub index: i32,
    pub pinyin: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flower {
    pub name: String,
    pub color: String,
    pub price: f32,
    pub line: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Furniture {
    pub id: i32,
    pub name: String,
    pub r#type: i32,
    pub color: i32,
    pub source: String,
    pub architecture: i32,
    pub limit: i32,
    pub quality: i32,
    pub view: i32,
    pub practical: i32,
    pub hard: i32,
    pub geomantic: i32,
    pub interesting: i32,
    pub produce: Option<String>,
    pub image: String,
    pub tip: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Travel {
    pub id: i32,
    pub name: String,
    pub r#type: i32,
    pub color: i32,
    pub source: String,
    pub architecture: i32,
    pub limit: i32,
    pub quality: i32,
    pub view: i32,
    pub practical: i32,
    pub hard: i32,
    pub geomantic: i32,
    pub interesting: i32,
    pub produce: Option<String>,
    pub image: String,
    pub tip: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct News {
    pub id: i32,
    pub token: i32,
    pub class: String,
    pub title: String,
    pub date: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Announce {
    id: i32,
    token: i32,
    class: String,
    title: String,
    date: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerMaster {
    id: String,
    zone: String,
    name: String,
    event: i32,
    voice: HashMap<String, Vec<i32>>,
    alias: Vec<String>,
    slave: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerCheck {
    pub id: i32,
    pub zone: String,
    pub server: String,
    pub status: i32,
    time: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerStatus {
    pub zone: String,
    pub server: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkillsRecord {
    pub id: String,
    pub title: String,
    pub url: String,
    pub time: String,
}
