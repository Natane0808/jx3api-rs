//! WebSocket 客户端：连接、自动重连、心跳与事件派发。
//!
//! ```no_run
//! use std::time::Duration;
//!
//! use jx3_api::socket::{SocketAction, SocketClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = SocketClient::builder()
//!         .token("jx3api::xxxxxxxx")
//!         .heartbeat(Duration::from_secs(30))
//!         .build()?;
//!
//!     // 只订阅「奇遇触发」。
//!     client.on(SocketAction::EventTrigger, |event| {
//!         println!("奇遇: {event:?}");
//!     });
//!     // 订阅全部事件。
//!     client.on_any(|event| println!("{}: {:?}", event.name(), event));
//!
//!     let handle = client.connect().await?;
//!     // ……业务处理……
//!     handle.close().await?;
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio::time::{MissedTickBehavior, interval, sleep, timeout};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

use crate::socket::error::{SocketError, SocketResult};
use crate::socket::events::{SocketAction, SocketEvent};

/// 默认的 WebSocket 服务地址。
pub const DEFAULT_SOCKET_URL: &str = "wss://socket.nicemoe.cn";

/// 默认的心跳间隔（30 秒）。
pub const DEFAULT_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);

/// 默认的最大连续重连次数。
pub const DEFAULT_MAX_RECONNECTS: usize = 10;

/// 默认的首次重连退避时间（1 秒）。
pub const DEFAULT_RECONNECT_BACKOFF: Duration = Duration::from_secs(1);

/// 默认的指数退避上限（60 秒）。
pub const DEFAULT_MAX_RECONNECT_BACKOFF: Duration = Duration::from_secs(60);

/// 默认的优雅关闭等待时间（5 秒）。
pub const DEFAULT_CLOSE_TIMEOUT: Duration = Duration::from_secs(5);

/// 事件回调类型：可跨线程共享，可被重复调用。
pub type SocketHandler = Arc<dyn Fn(SocketEvent) + Send + Sync>;

/// [`SocketClient`] 的构造器。
///
/// 所有配置都有默认值，最简单的用法是 `SocketClient::builder().build()?`。
#[derive(Debug, Clone)]
pub struct SocketBuilder {
    url: String,
    token: Option<String>,
    reconnect: bool,
    max_reconnects: usize,
    initial_backoff: Duration,
    max_backoff: Duration,
    heartbeat: Option<Duration>,
}

impl SocketBuilder {
    /// 创建构造器（全部使用默认值）。
    pub fn new() -> Self {
        Self {
            url: DEFAULT_SOCKET_URL.to_owned(),
            token: None,
            reconnect: true,
            max_reconnects: DEFAULT_MAX_RECONNECTS,
            initial_backoff: DEFAULT_RECONNECT_BACKOFF,
            max_backoff: DEFAULT_MAX_RECONNECT_BACKOFF,
            heartbeat: Some(DEFAULT_HEARTBEAT_INTERVAL),
        }
    }

    /// 设置服务地址，默认为 `wss://socket.nicemoe.cn`。
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /// 设置站点令牌，连接时会作为 `token` 查询参数附加到地址上。
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// 是否在连接断开后自动重连，默认开启。
    pub fn reconnect(mut self, enabled: bool) -> Self {
        self.reconnect = enabled;
        self
    }

    /// 设置最大连续重连次数，默认为 10；`0` 等价于不重连。
    pub fn max_reconnects(mut self, count: usize) -> Self {
        self.max_reconnects = count;
        self
    }

    /// 设置首次重连的退避时间，默认为 1 秒。
    pub fn backoff(mut self, backoff: Duration) -> Self {
        self.initial_backoff = backoff;
        self
    }

    /// 设置指数退避的上限，默认为 60 秒。
    pub fn max_backoff(mut self, max_backoff: Duration) -> Self {
        self.max_backoff = max_backoff;
        self
    }

    /// 设置心跳间隔，默认 30 秒；传入 `None` 可关闭心跳。
    ///
    /// 心跳通过向服务端发送 WebSocket `Ping` 帧实现。
    pub fn heartbeat(mut self, interval: impl Into<Option<Duration>>) -> Self {
        self.heartbeat = interval.into();
        self
    }

    /// 构建 [`SocketClient`]（不会发起网络连接）。
    ///
    /// # Errors
    ///
    /// 地址不是合法的 `ws://` / `wss://` URL 时返回 [`SocketError::InvalidUrl`]。
    pub fn build(self) -> SocketResult<SocketClient> {
        let url = self.url.trim().to_owned();
        let valid = match url.split_once("://") {
            Some((scheme, rest)) => matches!(scheme, "ws" | "wss") && !rest.is_empty(),
            None => false,
        };
        if !valid {
            return Err(SocketError::InvalidUrl(url));
        }

        let token = self.token.filter(|token| !token.trim().is_empty());
        let (shutdown, _receiver) = watch::channel(false);

        Ok(SocketClient {
            inner: Arc::new(SocketInner {
                url,
                token,
                reconnect: self.reconnect,
                max_reconnects: self.max_reconnects,
                initial_backoff: self.initial_backoff,
                max_backoff: self.max_backoff.max(self.initial_backoff),
                // 零间隔会让 `tokio::time::interval` panic，这里按「关闭心跳」处理。
                heartbeat: self.heartbeat.filter(|interval| !interval.is_zero()),
                handlers: RwLock::new(HashMap::new()),
                any_handlers: RwLock::new(Vec::new()),
                shutdown,
                closed: AtomicBool::new(false),
            }),
        })
    }
}

impl Default for SocketBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket 客户端。
///
/// 客户端本身只保存配置、订阅者与关闭状态，克隆代价很低；真正的网络循环在
/// [`SocketClient::connect`] 返回的 [`SocketHandle`] 背后的 tokio task 中运行。
#[derive(Clone)]
pub struct SocketClient {
    inner: Arc<SocketInner>,
}

impl SocketClient {
    /// 创建构造器。
    pub fn builder() -> SocketBuilder {
        SocketBuilder::new()
    }

    /// 当前配置的服务地址（不包含令牌）。
    pub fn url(&self) -> &str {
        &self.inner.url
    }

    /// 当前配置的站点令牌。
    pub fn token(&self) -> Option<&str> {
        self.inner.token.as_deref()
    }

    /// 是否已调用 [`SocketClient::close`] 主动关闭。
    pub fn is_closed(&self) -> bool {
        self.inner.closed.load(Ordering::SeqCst)
    }

    /// 订阅单个 action。
    ///
    /// 回调在后台任务的上下文中执行，**不要**在回调里阻塞或长时间占用线程；
    /// 同一个 action 可以注册多个回调，按注册顺序依次调用。
    pub fn on<F>(&self, action: SocketAction, handler: F) -> &Self
    where
        F: Fn(SocketEvent) + Send + Sync + 'static,
    {
        let mut handlers = self
            .inner
            .handlers
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        handlers.entry(action).or_default().push(Arc::new(handler));
        self
    }

    /// 订阅全部 action。
    ///
    /// 无论事件是否被 [`SocketClient::on`] 单独订阅，`on_any` 注册的回调都会收到。
    pub fn on_any<F>(&self, handler: F) -> &Self
    where
        F: Fn(SocketEvent) + Send + Sync + 'static,
    {
        let mut handlers = self
            .inner
            .any_handlers
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        handlers.push(Arc::new(handler));
        self
    }

    /// 建立 WebSocket 连接，并在 tokio 后台任务中启动运行循环。
    ///
    /// 本方法会等待首次握手完成，因此首次连接失败会直接返回错误；握手成功后
    /// 的断线重连由后台循环按指数退避处理（见 [`SocketBuilder::reconnect`]）。
    ///
    /// # Errors
    ///
    /// * 客户端已关闭 —— [`SocketError::Closed`]；
    /// * 首次握手失败 —— [`SocketError::Connect`]。
    pub async fn connect(&self) -> SocketResult<SocketHandle> {
        if self.is_closed() {
            return Err(SocketError::Closed("客户端已关闭，无法重新连接".to_owned()));
        }

        let url = self.inner.connect_url();
        let (stream, _response) = connect_async(url.as_str())
            .await
            .map_err(|err| SocketError::Connect(err.to_string()))?;

        let shutdown = self.inner.shutdown.subscribe();
        let inner = Arc::clone(&self.inner);
        let task = tokio::spawn(async move {
            run_loop(inner, stream, shutdown).await;
        });

        Ok(SocketHandle {
            client: self.clone(),
            task,
        })
    }

    /// 连接并阻塞等待运行循环结束（直到被 [`SocketClient::close`] 关闭，
    /// 或重连次数耗尽）。
    ///
    /// # Errors
    ///
    /// 与 [`SocketClient::connect`] 相同；后台任务 panic / 被 abort 时返回
    /// [`SocketError::Protocol`]。
    pub async fn run(&self) -> SocketResult<()> {
        self.connect().await?.join().await
    }

    /// 优雅关闭：通知后台循环发送 WebSocket Close 帧并退出。
    ///
    /// 本方法只负责发出关闭信号，不会等待后台任务结束；需要等待时请调用
    /// [`SocketHandle::close`]。
    pub fn close(&self) {
        if !self.inner.closed.swap(true, Ordering::SeqCst) {
            let _ = self.inner.shutdown.send_replace(true);
        }
    }
}

impl fmt::Debug for SocketClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SocketClient")
            .field("url", &self.inner.url)
            .field("reconnect", &self.inner.reconnect)
            .field("max_reconnects", &self.inner.max_reconnects)
            .field("heartbeat", &self.inner.heartbeat)
            .field("closed", &self.is_closed())
            .finish_non_exhaustive()
    }
}

impl Default for SocketClient {
    fn default() -> Self {
        SocketBuilder::new()
            .build()
            .expect("默认 WebSocket 配置不应失败")
    }
}

/// [`SocketClient::connect`] 返回的后台任务句柄。
#[derive(Debug)]
pub struct SocketHandle {
    client: SocketClient,
    task: JoinHandle<()>,
}

impl SocketHandle {
    /// 句柄对应的客户端（可继续用来注册回调或主动关闭）。
    pub fn client(&self) -> &SocketClient {
        &self.client
    }

    /// 主动关闭并等待后台任务退出。
    ///
    /// 最多等待 [`DEFAULT_CLOSE_TIMEOUT`]；超时后会 abort 后台任务并返回
    /// [`SocketError::Timeout`]。
    ///
    /// # Errors
    ///
    /// 关闭超时或后台任务异常退出。
    pub async fn close(self) -> SocketResult<()> {
        self.client.close();
        let SocketHandle { task, .. } = self;
        let mut task = task;
        match timeout(DEFAULT_CLOSE_TIMEOUT, &mut task).await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(err)) => Err(SocketError::Protocol(format!("后台任务异常退出: {err}"))),
            Err(_) => {
                task.abort();
                Err(SocketError::Timeout(DEFAULT_CLOSE_TIMEOUT))
            }
        }
    }

    /// 等待后台任务自然结束（不主动发送关闭信号）。
    ///
    /// # Errors
    ///
    /// 后台任务 panic 或被 abort 时返回 [`SocketError::Protocol`]。
    pub async fn join(self) -> SocketResult<()> {
        self.task
            .await
            .map_err(|err| SocketError::Protocol(format!("后台任务异常退出: {err}")))
    }

    /// 立即中止后台任务（不发送 Close 帧），用于进程退出等场景。
    pub fn abort(&self) {
        self.client.close();
        self.task.abort();
    }
}

/// 客户端内部共享状态。
struct SocketInner {
    url: String,
    token: Option<String>,
    reconnect: bool,
    max_reconnects: usize,
    initial_backoff: Duration,
    max_backoff: Duration,
    heartbeat: Option<Duration>,
    handlers: RwLock<HashMap<SocketAction, Vec<SocketHandler>>>,
    any_handlers: RwLock<Vec<SocketHandler>>,
    shutdown: watch::Sender<bool>,
    closed: AtomicBool,
}

impl SocketInner {
    /// 拼接带令牌的完整连接地址。
    fn connect_url(&self) -> String {
        let Some(token) = self.token.as_deref() else {
            return self.url.clone();
        };
        let encoded =
            serde_urlencoded::to_string([("token", token)]).unwrap_or_else(|_| String::new());
        if encoded.is_empty() {
            return self.url.clone();
        }
        let separator = if self.url.contains('?') { '&' } else { '?' };
        format!("{}{separator}{encoded}", self.url)
    }

    /// 依次调用「按 action 订阅」与「订阅全部」的回调。
    fn dispatch(&self, event: SocketEvent) {
        let action = event.action();
        let specific: Vec<SocketHandler> = {
            let handlers = self
                .handlers
                .read()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            handlers.get(&action).cloned().unwrap_or_default()
        };
        let any: Vec<SocketHandler> = {
            let handlers = self
                .any_handlers
                .read()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            handlers.clone()
        };

        for handler in specific {
            handler(event.clone());
        }
        for handler in any {
            handler(event.clone());
        }
    }
}

/// 单条连接的读取结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SessionEnd {
    /// 收到主动关闭信号。
    Closed,
    /// 连接断开（需要按配置决定是否重连）。
    Disconnected,
}

/// 后台运行循环：负责重连与退避。
async fn run_loop(
    inner: Arc<SocketInner>,
    first_stream: SocketStream,
    mut shutdown: watch::Receiver<bool>,
) {
    let mut pending = Some(first_stream);
    let mut reconnects = 0usize;
    let mut backoff = inner.initial_backoff;

    loop {
        if *shutdown.borrow() || inner.closed.load(Ordering::SeqCst) {
            // 连接刚建立就收到关闭信号时，仍然发送 Close 帧，保证优雅关闭。
            if let Some(mut stream) = pending.take() {
                let _ = stream.close(None).await;
            }
            return;
        }

        let stream = match pending.take() {
            Some(stream) => stream,
            None => match connect_async(inner.connect_url().as_str()).await {
                Ok((stream, _response)) => stream,
                Err(_) => {
                    if !sleep_before_retry(&inner, &mut shutdown, &mut reconnects, &mut backoff)
                        .await
                    {
                        mark_reconnect_exhausted(&inner, &mut shutdown);
                        return;
                    }
                    continue;
                }
            },
        };

        reconnects = 0;
        backoff = inner.initial_backoff;

        if run_session(&inner, stream, &mut shutdown).await == SessionEnd::Closed {
            return;
        }

        if !inner.reconnect {
            return;
        }
        if !sleep_before_retry(&inner, &mut shutdown, &mut reconnects, &mut backoff).await {
            mark_reconnect_exhausted(&inner, &mut shutdown);
            return;
        }
    }
}

/// 按指数退避等待下一次重连；返回 `false` 表示应当停止重连。
async fn sleep_before_retry(
    inner: &SocketInner,
    shutdown: &mut watch::Receiver<bool>,
    reconnects: &mut usize,
    backoff: &mut Duration,
) -> bool {
    if *reconnects >= inner.max_reconnects {
        return false;
    }
    *reconnects += 1;

    let wait = *backoff;
    *backoff = backoff.saturating_mul(2).min(inner.max_backoff);

    tokio::select! {
        _ = sleep(wait) => true,
        _ = shutdown.changed() => false,
    }
}

/// 重连循环结束时区分「收到关闭信号」与「重连次数耗尽」。
///
/// 后者把客户端置为终止状态（`is_closed() == true`），避免调用方误以为还能继续重连；
/// 由 [`SocketClient::close`] 触发的停止则保持原来的 `closed` 语义。
fn mark_reconnect_exhausted(inner: &SocketInner, shutdown: &mut watch::Receiver<bool>) {
    if !*shutdown.borrow() && !inner.closed.load(Ordering::SeqCst) {
        inner.closed.store(true, Ordering::SeqCst);
    }
}

/// 单条连接上的读取 / 心跳循环。
async fn run_session<S>(
    inner: &SocketInner,
    mut stream: WebSocketStream<S>,
    shutdown: &mut watch::Receiver<bool>,
) -> SessionEnd
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut heartbeat = inner.heartbeat.map(|period| {
        let mut ticker = interval(period);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
        ticker
    });
    // `interval` 的第一次 tick 会立即完成，这里先消费掉，避免刚连上就发心跳。
    if let Some(ticker) = heartbeat.as_mut() {
        ticker.tick().await;
    }
    let heartbeat_enabled = heartbeat.is_some();

    loop {
        if *shutdown.borrow() || inner.closed.load(Ordering::SeqCst) {
            let _ = stream.close(None).await;
            return SessionEnd::Closed;
        }

        tokio::select! {
            biased;
            _ = shutdown.changed() => {
                let _ = stream.close(None).await;
                return SessionEnd::Closed;
            }
            message = stream.next() => {
                match message {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(event) = serde_json::from_str::<SocketEvent>(text.as_str()) {
                            inner.dispatch(event);
                        }
                    }
                    Some(Ok(Message::Binary(data))) => {
                        if let Ok(text) = std::str::from_utf8(&data) {
                            if let Ok(event) = serde_json::from_str::<SocketEvent>(text) {
                                inner.dispatch(event);
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) => return SessionEnd::Disconnected,
                    Some(Ok(_)) => {}
                    Some(Err(_)) | None => return SessionEnd::Disconnected,
                }
            }
            _ = heartbeat_tick(&mut heartbeat), if heartbeat_enabled => {
                if stream.send(Message::Ping(Default::default())).await.is_err() {
                    return SessionEnd::Disconnected;
                }
            }
        }
    }
}

/// 等待下一次心跳；未配置心跳时永不完成。
async fn heartbeat_tick(heartbeat: &mut Option<tokio::time::Interval>) {
    match heartbeat {
        Some(ticker) => {
            ticker.tick().await;
        }
        None => std::future::pending::<()>().await,
    }
}

/// 具体连接类型：`tokio-tungstenite` 的 `MaybeTlsStream` 包装 TCP 连接。
type SocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    fn client() -> SocketClient {
        SocketClient::builder().build().expect("默认配置应当合法")
    }

    #[test]
    fn builder_defaults() {
        let client = client();
        assert_eq!(client.url(), DEFAULT_SOCKET_URL);
        assert!(client.token().is_none());
        assert!(client.inner.reconnect);
        assert_eq!(client.inner.max_reconnects, DEFAULT_MAX_RECONNECTS);
        assert_eq!(client.inner.heartbeat, Some(DEFAULT_HEARTBEAT_INTERVAL));
        assert_eq!(client.inner.initial_backoff, DEFAULT_RECONNECT_BACKOFF);
        assert_eq!(client.inner.max_backoff, DEFAULT_MAX_RECONNECT_BACKOFF);
        assert!(!client.is_closed());
    }

    #[test]
    fn builder_accepts_custom_configuration() {
        let client = SocketClient::builder()
            .url("ws://127.0.0.1:9000")
            .token("jx3api::abc")
            .reconnect(false)
            .max_reconnects(3)
            .backoff(Duration::from_millis(100))
            .max_backoff(Duration::from_secs(2))
            .heartbeat(None)
            .build()
            .unwrap();

        assert_eq!(client.url(), "ws://127.0.0.1:9000");
        assert_eq!(client.token(), Some("jx3api::abc"));
        assert!(!client.inner.reconnect);
        assert_eq!(client.inner.max_reconnects, 3);
        assert_eq!(client.inner.initial_backoff, Duration::from_millis(100));
        assert_eq!(client.inner.max_backoff, Duration::from_secs(2));
        assert!(client.inner.heartbeat.is_none());
    }

    #[test]
    fn connect_url_contains_encoded_token() {
        let client = SocketClient::builder()
            .url("wss://socket.nicemoe.cn")
            .token("jx3api::abc")
            .build()
            .unwrap();
        let url = client.inner.connect_url();
        assert!(url.starts_with("wss://socket.nicemoe.cn?"));
        assert!(url.contains("token=jx3api"));
        assert!(url.contains("%3A%3A"));
    }

    #[test]
    fn connect_url_keeps_existing_query() {
        let client = SocketClient::builder()
            .url("wss://socket.nicemoe.cn?foo=1")
            .token("abc")
            .build()
            .unwrap();
        let url = client.inner.connect_url();
        assert!(url.starts_with("wss://socket.nicemoe.cn?foo=1&token=abc"));
    }

    #[test]
    fn builder_rejects_invalid_url() {
        assert!(matches!(
            SocketClient::builder().url("https://example.com").build(),
            Err(SocketError::InvalidUrl(_))
        ));
        assert!(matches!(
            SocketClient::builder().url("ws://").build(),
            Err(SocketError::InvalidUrl(_))
        ));
        assert!(matches!(
            SocketClient::builder().url("socket.nicemoe.cn").build(),
            Err(SocketError::InvalidUrl(_))
        ));
    }

    #[test]
    fn handlers_receive_matching_events() {
        let client = client();
        let specific = Arc::new(AtomicUsize::new(0));
        let any = Arc::new(AtomicUsize::new(0));

        let specific_counter = Arc::clone(&specific);
        client.on(SocketAction::EventTrigger, move |_| {
            specific_counter.fetch_add(1, Ordering::SeqCst);
        });
        let any_counter = Arc::clone(&any);
        client.on_any(move |_| {
            any_counter.fetch_add(1, Ordering::SeqCst);
        });

        let trigger: SocketEvent = serde_json::from_str(
            r#"{"action":1001,"status":"success","detail":{"zone":"电信一区","server":"长安城","name":"叶英","event":"黑白路","time":1794106931}}"#,
        )
        .unwrap();
        client.inner.dispatch(trigger);
        assert_eq!(specific.load(Ordering::SeqCst), 1);
        assert_eq!(any.load(Ordering::SeqCst), 1);

        let status: SocketEvent = serde_json::from_str(
            r#"{"action":2001,"status":"success","detail":{"zone":"电信一区","server":"长安城","status":"开服","time":1794106931}}"#,
        )
        .unwrap();
        client.inner.dispatch(status);
        assert_eq!(specific.load(Ordering::SeqCst), 1);
        assert_eq!(any.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn close_marks_client_closed() {
        let client = client();
        assert!(!client.is_closed());
        client.close();
        assert!(client.is_closed());
    }

    #[test]
    fn reconnect_exhaustion_marks_client_closed() {
        let client = client();
        let (_shutdown, mut receiver) = watch::channel(false);
        assert!(!client.is_closed());
        mark_reconnect_exhausted(&client.inner, &mut receiver);
        assert!(client.is_closed(), "重连耗尽后应进入终止状态");
    }

    #[test]
    fn shutdown_signal_does_not_mark_exhausted() {
        let client = client();
        let (shutdown, mut receiver) = watch::channel(false);
        let _ = shutdown.send_replace(true);
        mark_reconnect_exhausted(&client.inner, &mut receiver);
        assert!(!client.is_closed(), "主动关闭不应被当作重连耗尽");
    }

    #[tokio::test]
    async fn closed_client_refuses_connect() {
        let client = client();
        client.close();
        match client.connect().await {
            Err(SocketError::Closed(_)) => {}
            other => panic!("应当返回 Closed 错误: {other:?}"),
        }
    }
}
