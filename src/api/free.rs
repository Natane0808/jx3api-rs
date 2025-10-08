use crate::Jx3ApiClient;
use crate::enums::{CelebClub, ServerCheckEnums};
use crate::err::Jx3ApiError;
use crate::model::ApiResp;
use crate::model::free::{
    Announce, Answer, Calendar, Celeb, Flower, Furniture, ListCalendar, News, ServerMaster,
    ServerStatus, SkillsRecord, Travel,
};
use serde_json::Value;
use std::collections::HashMap;

/// 活动日历
const CALENDAR: &'static str = "/active/calendar";
/// 活动月历
const LIST_CALENDAR: &'static str = "/active/list/calendar";

/// 行侠事件
const CELEBS: &'static str = "/active/celebs";

/// 科举答题
const ANSWER: &'static str = "/exam/answer";

///家园鲜花
const FLOWER: &'static str = "/home/flower";

///家园装饰
const FURNITURE: &'static str = "/home/furniture";

/// 器物图谱
const TRAVEL: &'static str = "/home/travel";

/// 新闻资讯
const ALL_NEWS: &'static str = "/news/allnews";

/// 维护公告
const ANNOUNCE: &'static str = "/news/announce";

/// 搜索区服
const SERVER_MASTER: &'static str = "/server/master";

/// 开服检查
const SERVER_CHECK: &'static str = "/server/check";

/// 查看状态
const SERVER_STATUS: &'static str = "/server/status";

/// 技改记录
const SKILLS_RECORDS: &'static str = "/skills/records";

impl Jx3ApiClient {
    /// # 获取活动日历
    /// 查询 今天、明天、后天 和 日常任务 的相关活动。
    /// - 参数说明:
    ///   - `server`:指定目标区服，获取该区服的美人图任务数据。
    ///   - `num`: 指定日期的偏移值
    ///     - 0:表示当天
    ///     - 1:表示明天
    ///     - 2:表示后天
    ///     - 默认值为 0，即查询当天的数据。
    ///
    ///
    /// - 返回示例:
    /// ```json
    ///   {
    ///   "code": 200,
    ///   "msg": "success",
    ///   "data": {
    ///     "date": "2024-12-08",
    ///     "week": "日",
    ///     "war": "大战！英雄冰川宫宝库",
    ///     "battle": "神农洇",
    ///     "orecar": "跨服·河西瀚漠",
    ///     "school": "明教·血染红衣",
    ///     "rescue": "少林·乱世",
    ///     "luck": [
    ///       "酒客",
    ///       "圆圆",
    ///       "鸿鸿"
    ///     ],
    ///     "card": [
    ///       "英雄天子峰",
    ///       "英雄日轮山城",
    ///       "英雄风雨稻香村",
    ///       "英雄剑冢"
    ///     ],
    ///     "team": [
    ///       "洞天福地·守卫泥兰;龙泉府·雪国冬猎",
    ///       "英雄雁门关之役;华清宫回忆录;英雄漳水南路",
    ///       "一之窟;九老洞;河阳之战"
    ///     ]
    ///   },
    ///   "time": 1733646979
    /// }
    /// ```
    /// - 注:
    ///   - 美人画图:仅在 星期三、星期五、星期六 和 星期日 有提供。若非活动时间，则不会返回相关数据。
    ///   - 世界首领:仅在 星期三、星期五 存在，其他时间不返回。
    pub async fn calendar(
        &self,
        server: Option<&str>,
        num: Option<u8>,
    ) -> Result<ApiResp<Calendar>, Jx3ApiError> {
        let full_url = format!("{}{}", self.base_url, CALENDAR);
        let mut body_map = HashMap::new();
        if let Some(server) = server {
            body_map.insert("server", Value::String(server.to_string()));
        }
        if let Some(num) = num {
            body_map.insert("num", Value::Number(num.into()));
        }

        let resp = self
            .client
            .post(full_url)
            .json(&body_map)
            .send()
            .await?
            .json::<ApiResp<Calendar>>()
            .await?;

        match resp.code {
            200 => Ok(resp),
            _ => Err(Jx3ApiError::ApiError(resp.code, resp.msg)),
        }
    }

    /// # 活动月历
    /// 该 API 用于预测每天的日常任务，您可以通过指定时间范围来获取相关任务数据。
    /// - 参数说明:
    ///   - `num`: 预测时间范围，返回指定日期的月历。默认值为 15，表示返回前后15天的日历数据。
    /// - 返回示例:
    /// ```json
    /// {
    ///   "code": 200,
    ///   "msg": "success",
    ///   "data": {
    ///     "today": {
    ///       "date": "2024-12-08",
    ///       "week": "日",
    ///       "year": "2024",
    ///       "month": "12",
    ///       "day": "08"
    ///     },
    ///     "data": [
    ///       {
    ///         "date": "2024-11-24",
    ///         "day": "24",
    ///         "week": "日",
    ///         "war": "英雄不染窟",
    ///         "battle": "浮香丘",
    ///         "orecar": "跨服·河西瀚漠",
    ///         "school": "唐门·对抗狼牙",
    ///         "rescue": "七秀·乱世",
    ///         "luck": [
    ///           "鸿鸿",
    ///           "将军客",
    ///           "财财"
    ///         ],
    ///         "card": [
    ///           "银雾湖",
    ///           "狼牙堡·战兽山",
    ///           "狼牙堡·燕然峰"
    ///         ]
    ///       }
    ///     ]
    ///   },
    ///   "time": 1733654875
    /// }
    /// ```
    ///

    pub async fn list_calendar(
        &self,
        num: Option<u32>,
    ) -> Result<ApiResp<ListCalendar>, Jx3ApiError> {
        let full_url = format!("{}{}", self.base_url, LIST_CALENDAR);
        let mut body_map = HashMap::new();
        if let Some(num) = num {
            body_map.insert("num", Value::Number(num.into()));
        }
        let resp = self
            .client
            .post(full_url)
            .json(&body_map)
            .send()
            .await?
            .json::<ApiResp<ListCalendar>>()
            .await?;
        match resp.code {
            200 => Ok(resp),
            _ => Err(Jx3ApiError::ApiError(resp.code, resp.msg)),
        }
    }

    /// # 行侠事件
    /// 查询当前时间的楚天社或云从社的进度。
    /// - 参数说明:
    ///   - `name`:指定目标名称，返回指定事件的信息。可以选择 "楚天社" 或 "云从社" 以及 "披风会"。
    /// - 返回示例:
    /// ```json
    /// {
    ///   "code": 200,
    ///   "msg": "success",
    ///   "data": [
    ///     {
    ///        "map": "河西瀚漠",
    ///        "stage": "并赃拿贼",
    ///        "site": "北丘墓",
    ///        "desc": "击退罗一空",
    ///        "icon": "382",
    ///        "time": "12:12"
    ///      },
    ///      {
    ///        "map": "河西瀚漠",
    ///        "stage": "淳朴民风",
    ///        "site": "高昌城",
    ///        "desc": "击败巴老九",
    ///        "icon": "382",
    ///        "time": "12:16"
    ///     },
    ///   ],
    ///   "time": 1725570373
    /// }
    /// ```
    pub async fn celebs(&self, name: CelebClub) -> Result<ApiResp<Vec<Celeb>>, Jx3ApiError> {
        let full_url = format!("{}{}", self.base_url, CELEBS);
        let mut body_map = HashMap::with_capacity(1);
        body_map.insert("name", Value::String(name.to_str().to_string()));
        let resp = self
            .client
            .post(full_url)
            .json(&body_map)
            .send()
            .await?
            .json::<ApiResp<Vec<Celeb>>>()
            .await?;
        match resp.code {
            200 => Ok(resp),
            _ => Err(Jx3ApiError::ApiError(resp.code, resp.msg)),
        }
    }

    /// # 科举答题
    /// 此接口用于科举考试时的答案查询，支持模糊查询和拼音首字母查询。
    /// - 参数说明:
    ///   - `subject`:输入科举试题的题目内容，支持模糊查询或拼音首字母查询。例如输入 古琴 或 GQ。
    ///   - `limit`:限制返回的答案数量，默认值为 10，取值范围为 1-20。
    /// - 返回示例:
    /// ```json
    /// {
    ///   "code": 200,
    ///   "msg": "success",
    ///   "data": [
    ///     {
    ///      "id": 885,
    ///       "question": "单选题：古琴有几根弦？",
    ///       "answer": "七根",
    ///      "correctness": 1,
    ///       "index": 2,
    ///       "pinyin": "DXTGQYJGX"
    ///     }
    ///   ],
    ///   "time": 1725571291
    /// }
    /// ```
    ///
    pub async fn answer(
        &self,
        subject: Option<&str>,
        limit: Option<u8>,
    ) -> Result<ApiResp<Vec<Answer>>, Jx3ApiError> {
        let full_url = format!("{}{}", self.base_url, ANSWER);
        let mut body_map = HashMap::new();
        if let Some(subject) = subject {
            body_map.insert("subject", Value::String(subject.to_string()));
        }
        if let Some(limit) = limit {
            body_map.insert("limit", Value::Number(limit.into()));
        }
        let resp = self
            .client
            .post(full_url)
            .json(&body_map)
            .send()
            .await?
            .json::<ApiResp<Vec<Answer>>>()
            .await?;
        match resp.code {
            200 => Ok(resp),
            _ => Err(Jx3ApiError::ApiError(resp.code, resp.msg)),
        }
    }

    /// # 家园鲜花
    /// 此接口用于查询家园系统中特定鲜花的最高价格及其对应的采集线路，包括区服和地图信息。
    /// - 参数说明:
    ///   - `server`:指定目标区服，查询目标区服的相关信息。
    ///   - `name`: 指定鲜花名称，查询目标鲜花的相关信息。如果未指定，将返回所有鲜花的记录。
    ///   - `map`: 指定目标地图，查询该地图的相关鲜花信息。如果未指定，将返回所有地图的记录。
    /// - 返回参数说明:
    ///   - `HashMap`的key为地图名称，value为该地图的鲜花列表。
    /// - 返回示例:
    /// ```json
    /// {
    ///   "code": 200,
    ///   "msg": "success",
    ///   "data": {
    ///     "九寨沟·镜海": [
    ///       {
    ///         "name": "二级绣球花",
    ///         "color": "金，蓝，粉",
    ///        "price": 1.5,
    ///         "line": [
    ///          "11",
    ///           "16",
    ///          "10"
    ///         ]
    ///      }
    ///     ]
    ///   },
    ///   "time": 1723007315
    /// }
    /// ```

    pub async fn flower(
        &self,
        server: &str,
        name: Option<&str>,
        map: Option<&str>,
    ) -> Result<ApiResp<HashMap<String, Vec<Flower>>>, Jx3ApiError> {
        let full_url = format!("{}{}", self.base_url, FLOWER);
        let mut body_map = HashMap::new();
        body_map.insert("server", Value::String(server.to_string()));
        if let Some(name) = name {
            body_map.insert("name", Value::String(name.to_string()));
        }
        if let Some(map) = map {
            body_map.insert("map", Value::String(map.to_string()));
        }
        let resp = self
            .client
            .post(full_url)
            .json(&body_map)
            .send()
            .await?
            .json::<ApiResp<HashMap<String, Vec<Flower>>>>()
            .await?;
        match resp.code {
            200 => Ok(resp),
            _ => Err(Jx3ApiError::ApiError(resp.code, resp.msg)),
        }
    }

    /// # 家园装饰
    ///
    /// 此接口用于查询家园系统中特定装饰的详细信息，包括来源、属性和外观等内容。
    ///
    /// - 参数说明:
    ///   - `name`: 指定目标装饰的名称，查找该装饰的详细信息。
    ///
    /// - 返回示例:
    /// ```json
    /// {
    ///   "code": 200,
    ///   "msg": "success",
    ///   "data": {
    ///     "id": 1,
    ///     "name": "龙门香梦",
    ///     "type": 1,
    ///     "color": 4,
    ///     "source": "园宅会赛",
    ///     "architecture": 0,
    ///     "limit": 10,
    ///     "quality": 1000,
    ///     "view": 7931,
    ///     "practical": 3525,
    ///     "hard": 3525,
    ///     "geomantic": 3525,
    ///     "interesting": 3525,
    ///     "produce": null,
    ///     "image": "https://dl.pvp.xoyo.com/prod/icons/ui/image/homeland/data/source/home/furniture/cloth/cloth5_4_1001.mesh.png",
    ///     "tip": "跳影如流泉，香梦过龙门。山中逍遥客，化为唤雨人。"
    ///   },
    ///  "time": 1723005805
    /// }
    ///
    pub async fn furniture(&self, name: &str) -> Result<ApiResp<Vec<Furniture>>, Jx3ApiError> {
        let full_url = format!("{}{}", self.base_url, FURNITURE);
        let mut body_map = HashMap::new();
        body_map.insert("name", Value::String(name.to_string()));
        let resp = self
            .client
            .post(full_url)
            .json(&body_map)
            .send()
            .await?
            .json::<ApiResp<Vec<Furniture>>>()
            .await?;
        match resp.code {
            200 => Ok(resp),
            _ => Err(Jx3ApiError::ApiError(resp.code, resp.msg)),
        }
    }

    /// # 器物图谱
    /// 此接口用于查询指定地图的装饰物品产出信息，包括装饰的属性、来源和外观等内容。
    ///
    /// - 参数说明:
    ///   - `name`:指定目标地图的名称，查询该地图的装饰物品产出信息。
    ///
    /// - 返回示例:
    ///
    /// ```json
    /// {
    ///   "code": 200,
    ///   "msg": "success",
    ///   "data": [
    ///     {
    ///       "id": 422,
    ///       "name": "日晷",
    ///       "type": 1,
    ///       "color": 3,
    ///      "source": "宠物游历",
    ///       "architecture": 0,
    ///       "limit": 3,
    ///       "quality": 615,
    ///       "view": 484,
    ///       "practical": 1743,
    ///       "hard": 500,
    ///       "geomantic": 500,
    ///      "interesting": 0,
    ///       "produce": "万花",
    ///       "image": "https://dl.pvp.xoyo.com/prod/icons/ui/image/homeland/data/source/home/furniture/ornament/ornament1_1_1008_p.prefab.png",
    ///       "tip": "产出地图：万花"
    ///     }
    ///   ],
    ///   "time": 1723006304
    /// }
    ///
    pub async fn travel(&self, name: &str) -> Result<ApiResp<Vec<Travel>>, Jx3ApiError> {
        let full_url = format!("{}{}", self.base_url, TRAVEL);
        let mut body_map = HashMap::new();
        body_map.insert("name", Value::String(name.to_string()));
        let resp = self
            .client
            .post(full_url)
            .json(&body_map)
            .send()
            .await?
            .json::<ApiResp<Vec<Travel>>>()
            .await?;
        match resp.code {
            200 => Ok(resp),
            _ => Err(Jx3ApiError::ApiError(resp.code, resp.msg)),
        }
    }

    /// # 新闻资讯
    /// 此接口用于获取官方最新公告及新闻内容，包括新闻分类、标题及链接等信息。
    ///
    /// - 参数说明:
    ///   - `limit`:  限制返回的新闻条数，默认值为 10，可选范围为 1-50。
    ///
    /// - 返回示例:
    /// ```json
    /// {
    ///   "code": 200,
    ///   "msg": "success",
    ///   "data": [
    ///     {
    ///       "id": 2612,
    ///       "token": 1334243,
    ///       "class": "官方公告",
    ///       "title": "12月9日例行维护公告",
    ///       "date": "12/08",
    ///       "url": "https://jx3.xoyo.com/announce/gg.html?id=1334243"
    ///     }
    ///   ],
    ///   "time": 1733665940
    /// }
    /// ```
    ///
    pub async fn all_news(&self, limit: Option<u8>) -> Result<ApiResp<Vec<News>>, Jx3ApiError> {
        let full_url = format!("{}{}", self.base_url, ALL_NEWS);
        let mut body_map = HashMap::new();
        if let Some(limit) = limit {
            body_map.insert("limit", Value::Number(limit.into()));
        }
        let resp = self
            .client
            .post(full_url)
            .json(&body_map)
            .send()
            .await?
            .json::<ApiResp<Vec<News>>>()
            .await?;
        match resp.code {
            200 => Ok(resp),
            _ => Err(Jx3ApiError::ApiError(resp.code, resp.msg)),
        }
    }

    /// # 维护公告
    /// 此接口用于获取官方最新维护公告，包括公告标题、发布日期及链接等信息。
    /// - 参数说明:
    ///   - `limit`:  限制返回的维护公告条数，默认值为 10，可选范围为 1-50。
    /// - 返回示例:
    /// ```json
    /// {
    ///   "code": 200,
    ///   "msg": "success",
    ///   "data": [
    ///     {
    ///       "id": 2607,
    ///       "token": 1334235,
    ///       "class": "官方公告",
    ///       "title": "12月5日1.0.0.8799版本更新公告",
    ///       "date": "12/05",
    ///       "url": "https://jx3.xoyo.com/announce/gg.html?id=1334235"
    ///     }
    ///   ],
    ///   "time": 1733666196
    /// }
    /// ```
    pub async fn announce(&self, limit: Option<u8>) -> Result<ApiResp<Vec<Announce>>, Jx3ApiError> {
        let full_url = format!("{}{}", self.base_url, ANNOUNCE);
        let mut body_map = HashMap::new();
        if let Some(limit) = limit {
            body_map.insert("limit", Value::Number(limit.into()));
        }
        let resp = self
            .client
            .post(full_url)
            .json(&body_map)
            .send()
            .await?
            .json::<ApiResp<Vec<Announce>>>()
            .await?;
        match resp.code {
            200 => Ok(resp),
            _ => Err(Jx3ApiError::ApiError(resp.code, resp.msg)),
        }
    }

    /// # 搜索区服
    /// 此接口用于根据区服简称搜索主服务器及其附属服务器的详细信息。
    ///
    /// - 参数说明:
    ///   - `name`:指定目标区服的简称或名称，查询其主次服务器的详细信息。
    ///
    /// - 返回示例:
    /// ```json
    /// {
    ///   "code": 200,
    ///   "msg": "success",
    ///   "data": {
    ///     "id": "0502",
    ///     "zone": "电信区",
    ///     "name": "梦江南",
    ///     "event": 36,
    ///     "voice": {
    ///       "浩气盟": [32968],
    ///       "恶人谷": [36911]
    ///     },
    ///     "alias": [
    ///       "双梦镇",
    ///       "双梦"
    ///     ],
    ///     "slave": [
    ///      "梦江南",
    ///      "枫泾古镇",
    ///       "如梦令"
    ///     ]
    ///   },
    ///   "time": 1759917509
    /// }
    /// ```
    pub async fn server_master(&self, name: &str) -> Result<ApiResp<ServerMaster>, Jx3ApiError> {
        let full_url = format!("{}{}", self.base_url, SERVER_MASTER);
        let mut body_map = HashMap::new();
        body_map.insert("name", Value::String(name.to_string()));
        let resp = self
            .client
            .post(full_url)
            .json(&body_map)
            .send()
            .await?
            .json::<ApiResp<ServerMaster>>()
            .await?;
        match resp.code {
            200 => Ok(resp),
            _ => Err(Jx3ApiError::ApiError(resp.code, resp.msg)),
        }
    }

    /// # 开服检查
    /// 此接口用于查询指定服务器的当前状态，支持返回区服是否开服或维护中的信息。
    /// - 参数说明:
    ///   - `server`:指定目标服务名称，查询其当前状态。若未输入区服名称或输入错误的区服名称，将返回所有区服的状态数据，可用于开服监控（支持轮询请求）。
    /// - 返回示例:
    ///   - 示例1:
    /// ```json
    /// {
    ///    "code": 200,
    ///    "msg": "success",
    ///    "data": {
    ///      "id": 1,
    ///      "zone": "电信区",
    ///      "server": "长安城",
    ///      "status": 1,
    ///      "time": 1722977456
    ///    },
    ///    "time": 1723006935
    ///  }
    /// ```
    ///   - 示例2:
    /// ```json
    /// {
    ///   "code": 200,
    ///   "msg": "success",
    ///   "data": [
    ///     {
    ///       "id": 1,
    ///       "zone": "电信区",
    ///       "server": "长安城",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     },
    ///     {
    ///       "id": 2,
    ///       "zone": "电信区",
    ///       "server": "龙争虎斗",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     },
    ///     {
    ///       "id": 3,
    ///       "zone": "电信区",
    ///       "server": "蝶恋花",
    ///       "status": 1,
    ///       "time": 1759709301
    ///     },
    ///     {
    ///       "id": 4,
    ///       "zone": "电信区",
    ///       "server": "剑胆琴心",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     },
    ///     {
    ///       "id": 5,
    ///       "zone": "电信区",
    ///       "server": "幽月轮",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     },
    ///     {
    ///       "id": 6,
    ///       "zone": "电信区",
    ///       "server": "乾坤一掷",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     },
    ///     {
    ///       "id": 7,
    ///       "zone": "电信区",
    ///       "server": "斗转星移",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     },
    ///     {
    ///       "id": 8,
    ///       "zone": "电信区",
    ///       "server": "唯我独尊",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     },
    ///     {
    ///       "id": 9,
    ///       "zone": "电信区",
    ///       "server": "梦江南",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     },
    ///     {
    ///       "id": 10,
    ///       "zone": "电信区",
    ///       "server": "绝代天骄",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     },
    ///     {
    ///       "id": 11,
    ///       "zone": "双线区",
    ///       "server": "破阵子",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     },
    ///     {
    ///       "id": 12,
    ///       "zone": "双线区",
    ///       "server": "天鹅坪",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     },
    ///     {
    ///       "id": 13,
    ///       "zone": "双线区",
    ///       "server": "飞龙在天",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     },
    ///     {
    ///       "id": 14,
    ///       "zone": "双线区",
    ///       "server": "青梅煮酒",
    ///       "status": 0,
    ///       "time": 1745447709
    ///     },
    ///     {
    ///       "id": 15,
    ///       "zone": "缘起区",
    ///       "server": "缘起稻香",
    ///       "status": 1,
    ///       "time": 1759705416
    ///     },
    ///     {
    ///       "id": 16,
    ///       "zone": "缘起区",
    ///       "server": "天宝盛世",
    ///       "status": 1,
    ///       "time": 1759705417
    ///     },
    ///     {
    ///       "id": 17,
    ///       "zone": "无界区",
    ///       "server": "有人赴约",
    ///       "status": 0,
    ///       "time": 1745447709
    ///     },
    ///     {
    ///       "id": 18,
    ///       "zone": "无界区",
    ///       "server": "山海相逢",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     },
    ///     {
    ///       "id": 20,
    ///       "zone": "无界区",
    ///       "server": "万象长安",
    ///       "status": 0,
    ///       "time": 1745447709
    ///     },
    ///     {
    ///       "id": 21,
    ///       "zone": "无界区",
    ///       "server": "眉间雪",
    ///       "status": 1,
    ///       "time": 1759709302
    ///     }
    ///   ],
    ///   "time": 1759918555
    /// }
    /// ```

    pub async fn server_check(
        &self,
        server: Option<&str>,
    ) -> Result<ApiResp<ServerCheckEnums>, Jx3ApiError> {
        let full_url = format!("{}{}", self.base_url, SERVER_CHECK);
        let mut body_map = HashMap::new();
        if let Some(server) = server {
            body_map.insert("server", Value::String(server.to_string()));
        }
        let resp = self
            .client
            .post(full_url)
            .json(&body_map)
            .send()
            .await?
            .json::<ApiResp<ServerCheckEnums>>()
            .await?;
        match resp.code {
            200 => Ok(resp),
            _ => Err(Jx3ApiError::ApiError(resp.code, resp.msg)),
        }
    }

    /// # 查看状态
    /// 此接口用于查询指定服务器的当前状态，包括 维护、正常、繁忙 和 爆满 等状态信息。
    /// - 参数说明:
    ///   - `server`: 指定目标区服的名称，查询该区服的状态信息。
    /// - 返回示例:
    /// ```json
    /// {
    ///   "code": 200,
    ///   "msg": "success",
    ///   "data": {
    ///     "zone": "电信区",
    ///     "server": "长安城",
    ///     "status": "爆满"
    ///   },
    ///   "time": 1723007064
    /// }
    /// ```
    pub async fn server_status(&self, server: &str) -> Result<ApiResp<ServerStatus>, Jx3ApiError> {
        let full_url = format!("{}{}", self.base_url, SERVER_STATUS);
        let mut body_map = HashMap::new();
        body_map.insert("server", Value::String(server.to_string()));
        let resp = self
            .client
            .post(full_url)
            .json(&body_map)
            .send()
            .await?
            .json::<ApiResp<ServerStatus>>()
            .await?;
        match resp.code {
            200 => Ok(resp),
            _ => Err(Jx3ApiError::ApiError(resp.code, resp.msg)),
        }
    }

    /// # 技改记录
    /// 此接口用于查询技能的历史修改记录，包括资料片更新、技能调整等信息。
    /// - 返回示例:
    /// ```json
    /// {
    ///   "code": 200,
    ///   "msg": "success",
    ///   "data": [
    ///     {
    ///      "id": "1334209",
    ///      "title": "11月21日“丝路风语”资料片武学调整",
    ///      "url": "https://jx3.xoyo.com/announce/gg.html?id=1334209",
    ///      "time": "2024-11-20 23:06:15"
    ///    }
    ///   ],
    ///  "time": 1733668273
    /// }
    /// ```
    pub async fn skills_records(&self) -> Result<ApiResp<Vec<SkillsRecord>>, Jx3ApiError> {
        let full_url = format!("{}{}", self.base_url, SKILLS_RECORDS);
        let resp = self
            .client
            .get(full_url)
            .send()
            .await?
            .json::<ApiResp<Vec<SkillsRecord>>>()
            .await?;
        match resp.code {
            200 => Ok(resp),
            _ => Err(Jx3ApiError::ApiError(resp.code, resp.msg)),
        }
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[tokio::test]
    async fn test_calendar_example() -> Result<(), Jx3ApiError> {
        let client = Jx3ApiClient::new();
        let resp = client.calendar(Some("长安城"), Some(1)).await?;
        println!("{:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_list_calendar_example() -> Result<(), Jx3ApiError> {
        let client = Jx3ApiClient::new();
        let resp = client.list_calendar(Some(7)).await?;
        println!("{:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_celebs_example() -> Result<(), Jx3ApiError> {
        let client = Jx3ApiClient::new();
        let resp = client.celebs(CelebClub::Cts).await?;
        println!("{:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_answer_example() -> Result<(), Jx3ApiError> {
        let client = Jx3ApiClient::new();
        let resp = client.answer(Some("古琴"), Some(10)).await?;
        println!("{:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_flower_example() -> Result<(), Jx3ApiError> {
        let client = Jx3ApiClient::new();
        let resp = client
            .flower("梦江南", Some("绣球花"), Some("九寨沟·镜海"))
            .await?;
        println!("{:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_furniture_example() -> Result<(), Jx3ApiError> {
        let client = Jx3ApiClient::new();
        let resp = client.furniture("龙门香梦").await?;
        println!("{:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_travel_example() -> Result<(), Jx3ApiError> {
        let client = Jx3ApiClient::new();
        let resp = client.travel("万花").await?;
        println!("{:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_all_news_example() -> Result<(), Jx3ApiError> {
        let client = Jx3ApiClient::new();
        let resp = client.all_news(Some(1)).await?;
        println!("{:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_announce_example() -> Result<(), Jx3ApiError> {
        let client = Jx3ApiClient::new();
        let resp = client.announce(Some(1)).await?;
        println!("{:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_master_example() -> Result<(), Jx3ApiError> {
        let client = Jx3ApiClient::new();
        let resp = client.server_master("双梦镇").await?;
        println!("{:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_server_check_example() -> Result<(), Jx3ApiError> {
        let client = Jx3ApiClient::new();
        let resp = client.server_check(Some("长安城")).await?;
        println!("{:?}", resp);
        let resp = client.server_check(None).await?;
        println!("{:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_server_status_example() -> Result<(), Jx3ApiError> {
        let client = Jx3ApiClient::new();
        let resp = client.server_status("长安城").await?;
        println!("{:?}", resp);
        Ok(())
    }

    #[tokio::test]
    async fn test_skills_records_example() -> Result<(), Jx3ApiError> {
        let client = Jx3ApiClient::new();
        let resp = client.skills_records().await?;
        println!("{:?}", resp);
        Ok(())
    }
}
