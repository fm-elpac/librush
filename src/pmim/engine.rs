use xkeysym::{KeyCode, Keysym};
use zbus::{fdo, SignalContext};

use super::server::Pmims;
use crate::ibus::{IBusEngine, IBusFactory, IBusModifierState};

#[derive(Debug, Clone)]
pub struct PmimEngine {
    s: Pmims,
}

impl PmimEngine {
    pub fn new(s: Pmims) -> Self {
        Self { s }
    }
}

impl IBusEngine for PmimEngine {
    async fn process_key_event(
        &mut self,
        sc: SignalContext<'_>,
        keyval: Keysym,
        keycode: KeyCode,
        state: IBusModifierState,
    ) -> fdo::Result<bool> {
        self.s
            .process_key_event(sc, keyval.into(), keycode.into(), state.raw_value())
            .await
    }

    async fn set_cursor_location(
        &mut self,
        sc: SignalContext<'_>,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    ) -> fdo::Result<()> {
        self.s.set_cursor_location(sc, x, y, w, h).await
    }

    async fn focus_in(&mut self, sc: SignalContext<'_>) -> fdo::Result<()> {
        self.s.focus_in(sc).await
    }

    async fn focus_out(&mut self, sc: SignalContext<'_>) -> fdo::Result<()> {
        self.s.focus_out(sc).await
    }

    async fn reset(&mut self, sc: SignalContext<'_>) -> fdo::Result<()> {
        self.s.reset(sc).await
    }

    async fn enable(&mut self, sc: SignalContext<'_>) -> fdo::Result<()> {
        self.s.enable(sc).await
    }

    async fn disable(&mut self, sc: SignalContext<'_>) -> fdo::Result<()> {
        self.s.disable(sc).await
    }
}

#[derive(Debug, Clone)]
pub struct PmimFactory {
    s: Pmims,
}

impl PmimFactory {
    pub fn new(s: Pmims) -> Self {
        Self { s }
    }
}

impl IBusFactory<PmimEngine> for PmimFactory {
    fn create_engine(&mut self, name: String) -> Result<PmimEngine, String> {
        let 名称 = "pmim";

        if 名称 == name {
            Ok(PmimEngine::new(self.s.clone()))
        } else {
            Err(format!("unknown name: {}", name))
        }
    }
}
