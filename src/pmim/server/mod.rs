//! pmim-server 接口 (unix socket)
use std::error::Error;
use tokio::sync::{mpsc, oneshot};
use zbus::{fdo, SignalContext};

mod at;
mod m;

use at::{at_k, at_r, at_s};
use m::{MSender, Mk, Mr, Ms, MsC, MsK, MsS};

#[derive(Debug, Clone)]
pub struct Pmims {
    s: MSender<Ms>,
    k: mpsc::Sender<Mk>,
    r: mpsc::Sender<Mr>,
}

impl Pmims {
    pub fn new(s: MSender<Ms>, k: mpsc::Sender<Mk>, r: mpsc::Sender<Mr>) -> Self {
        Self { s, k, r }
    }

    async fn set_sc(&mut self, sc: SignalContext<'_>) {
        // TODO 更好的错误处理
        // 忽略错误
        let _ = self.r.send(Mr::SC(sc.to_owned())).await;
    }

    /// 发送 `Ms` 消息
    async fn send(&self, m: Ms) -> fdo::Result<()> {
        self.s
            .send(m)
            .await
            .map_err(|e| fdo::Error::Failed(format!("{:?}", e)))
    }

    /// 发送 `Mk` 消息
    async fn send_k(&self, m: Mk) {
        // TODO 更好的错误处理
        // 忽略错误
        let _ = self.k.send(m).await;
    }

    pub async fn process_key_event(
        &mut self,
        sc: SignalContext<'_>,
        keyval: u32,
        keycode: u32,
        state: u32,
    ) -> fdo::Result<bool> {
        self.set_sc(sc).await;
        self.send(Ms::K(MsK::new(keyval, keycode, state))).await?;

        let mut 捕捉 = false;
        let (tx, rx) = oneshot::channel();

        self.send_k(Mk::ProcessKeyEvent((keyval, keycode, state, tx)))
            .await;

        // 忽略错误
        if let Ok(b) = rx.await {
            捕捉 = b;
        }
        Ok(捕捉)
    }

    pub async fn set_cursor_location(
        &mut self,
        sc: SignalContext<'_>,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    ) -> fdo::Result<()> {
        self.set_sc(sc).await;
        self.send(Ms::C(MsC::new(x, y, w, h))).await
    }

    pub async fn focus_in(&mut self, sc: SignalContext<'_>) -> fdo::Result<()> {
        self.set_sc(sc).await;
        self.send_k(Mk::FocusIn).await;
        self.send(Ms::S(MsS("focus_in".to_string()))).await
    }

    pub async fn focus_out(&mut self, sc: SignalContext<'_>) -> fdo::Result<()> {
        self.set_sc(sc).await;
        self.send_k(Mk::FocusOut).await;
        self.send(Ms::S(MsS("focus_out".to_string()))).await
    }

    pub async fn reset(&mut self, sc: SignalContext<'_>) -> fdo::Result<()> {
        self.set_sc(sc).await;
        self.send_k(Mk::Reset).await;
        self.send(Ms::S(MsS("reset".to_string()))).await
    }

    pub async fn enable(&mut self, sc: SignalContext<'_>) -> fdo::Result<()> {
        self.set_sc(sc).await;
        self.send_k(Mk::Enable).await;
        self.send(Ms::S(MsS("enable".to_string()))).await
    }

    pub async fn disable(&mut self, sc: SignalContext<'_>) -> fdo::Result<()> {
        self.set_sc(sc).await;
        self.send_k(Mk::Disable).await;
        self.send(Ms::S(MsS("disable".to_string()))).await
    }
}

pub async fn 初始化pmims(flatpak: bool) -> Result<Pmims, Box<dyn Error>> {
    // 启动接收消息 (中转) 任务
    let sr = at_r();
    // 启动给 pmim-server 发送消息的任务
    let s = at_s(sr.clone(), flatpak)?;
    // 启动按键管理器
    let k = at_k(s.clone());

    // 将按键管理器 (消息发送端) 发送给 中转任务
    // 忽略错误
    let _ = sr.send(Mr::K(k.clone())).await;

    Ok(Pmims::new(s, k, sr))
}
