//! 各业务域的数据模型。
//!
//! 模型字段与官方 OpenAPI 的实际响应保持一致，并对缺失字段做了容错处理
//! （缺失时取默认值），避免官方字段调整导致整个响应解析失败。

pub mod active;
pub mod arena;
pub mod auction;
pub mod battle;
pub mod calcd;
pub mod card;
pub mod castle;
pub mod chat;
pub mod chitu;
pub mod duowan;
pub mod event;
pub mod exam;
pub mod firework;
pub mod food;
pub mod fraud;
pub mod home;
pub mod mech;
pub mod mentor;
pub mod monster;
pub mod news;
pub mod ranch;
pub mod rank;
pub mod recruit;
pub mod reward;
pub mod role;
pub mod sand;
pub mod saohua;
pub mod school;
pub mod server;
pub mod skill;
pub mod sound;
pub mod steed;
pub mod tieba;
pub mod trade;
pub mod trial;
pub mod tuilan;
pub mod wicked;

macro_rules! reexport {
    ($($module:ident),* $(,)?) => {
        $(
            pub use $module::*;
        )*
    };
}

reexport!(
    active, arena, auction, battle, calcd, card, castle, chat, chitu, duowan, event, exam,
    firework, food, fraud, home, mech, mentor, monster, news, ranch, rank, recruit, reward, role,
    sand, saohua, school, server, skill, sound, steed, tieba, trade, trial, tuilan, wicked,
);
