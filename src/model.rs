
pub mod free;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResp<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
    pub time: i32,
}
