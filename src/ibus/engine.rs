//! `ibus_engine` 表示一个输入法实现
use std::error::Error;
use std::future::Future;
use std::marker::Send;

use log::info;
use zbus::{fdo, interface, zvariant::Value, Connection, SignalContext};

/// Implement this trait to implement a input method
pub trait IBusEngine: Send + Sync {
    /// 键盘按键消息
    fn process_key_event(
        &mut self,
        _sc: SignalContext<'_>,
        _keyval: u32,
        _keycode: u32,
        _state: u32,
    ) -> impl Future<Output = fdo::Result<bool>> + Send {
        async { Ok(false) }
    }

    /// 设置光标位置
    fn set_cursor_location(
        &mut self,
        _sc: SignalContext<'_>,
        _x: i32,
        _y: i32,
        _w: i32,
        _h: i32,
    ) -> impl Future<Output = fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// 获得焦点
    fn focus_in(&mut self, _sc: SignalContext<'_>) -> impl Future<Output = fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// 失去焦点
    fn focus_out(
        &mut self,
        _sc: SignalContext<'_>,
    ) -> impl Future<Output = fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// 重置
    fn reset(&mut self, _sc: SignalContext<'_>) -> impl Future<Output = fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// 启用
    fn enable(&mut self, _sc: SignalContext<'_>) -> impl Future<Output = fdo::Result<()>> + Send {
        async { Ok(()) }
    }

    /// 禁用
    fn disable(&mut self, _sc: SignalContext<'_>) -> impl Future<Output = fdo::Result<()>> + Send {
        async { Ok(()) }
    }
}

/// D-Bus interface: `org.freedesktop.IBus.Engine`
///
/// <https://ibus.github.io/docs/ibus-1.5/IBusEngine.html>
#[derive(Debug, Clone)]
pub struct Engine<T: IBusEngine + 'static> {
    e: T,

    _op: String,
}

// 源文件: `ibus/src/ibusengine.c`
//
// <node>
//   <interface name='org.freedesktop.IBus.Engine'>
//
//     <method name='ProcessKeyEvent'>
//       <arg direction='in'  type='u' name='keyval' />
//       <arg direction='in'  type='u' name='keycode' />
//       <arg direction='in'  type='u' name='state' />
//       <arg direction='out' type='b' />
//     </method>
//     <method name='SetCursorLocation'>
//       <arg direction='in'  type='i' name='x' />
//       <arg direction='in'  type='i' name='y' />
//       <arg direction='in'  type='i' name='w' />
//       <arg direction='in'  type='i' name='h' />
//     </method>
//     <method name='ProcessHandWritingEvent'>
//       <arg direction='in'  type='ad' name='coordinates' />
//     </method>
//     <method name='CancelHandWriting'>
//       <arg direction='in'  type='u' name='n_strokes' />
//     </method>
//     <method name='SetCapabilities'>
//       <arg direction='in'  type='u' name='caps' />
//     </method>
//     <method name='PropertyActivate'>
//       <arg direction='in'  type='s' name='name' />
//       <arg direction='in'  type='u' name='state' />
//     </method>
//     <method name='PropertyShow'>
//       <arg direction='in'  type='s' name='name' />
//     </method>
//     <method name='PropertyHide'>
//       <arg direction='in'  type='s' name='name' />
//     </method>
//     <method name='CandidateClicked'>
//       <arg direction='in'  type='u' name='index' />
//       <arg direction='in'  type='u' name='button' />
//       <arg direction='in'  type='u' name='state' />
//     </method>
//     <method name='FocusIn' />
//     <method name='FocusInId'>
//       <arg direction='in'  type='s' name='object_path' />
//       <arg direction='in'  type='s' name='client' />
//     </method>
//     <method name='FocusIn' />
//     <method name='FocusOut' />
//     <method name='FocusOutId'>
//       <arg direction='in'  type='s' name='object_path' />
//     </method>
//     <method name='Reset' />
//     <method name='Enable' />
//     <method name='Disable' />
//     <method name='PageUp' />
//     <method name='PageDown' />
//     <method name='CursorUp' />
//     <method name='CursorDown' />
//     <method name='SetSurroundingText'>
//       <arg direction='in'  type='v' name='text' />
//       <arg direction='in'  type='u' name='cursor_pos' />
//       <arg direction='in'  type='u' name='anchor_pos' />
//     </method>
//     <method name='PanelExtensionReceived'>
//       <arg direction='in'  type='v' name='event' />
//     </method>
//     <method name='PanelExtensionRegisterKeys'>
//       <arg direction='in'  type='v' name='data' />
//     </method>
//
//     <signal name='CommitText'>
//       <arg type='v' name='text' />
//     </signal>
//     <signal name='UpdatePreeditText'>
//       <arg type='v' name='text' />
//       <arg type='u' name='cursor_pos' />
//       <arg type='b' name='visible' />
//       <arg type='u' name='mode' />
//     </signal>
//     <signal name='UpdateAuxiliaryText'>
//       <arg type='v' name='text' />
//       <arg type='b' name='visible' />
//     </signal>
//     <signal name='UpdateLookupTable'>
//       <arg type='v' name='table' />
//       <arg type='b' name='visible' />
//     </signal>
//     <signal name='RegisterProperties'>
//       <arg type='v' name='props' />
//     </signal>
//     <signal name='UpdateProperty'>
//       <arg type='v' name='prop' />
//     </signal>
//     <signal name='ForwardKeyEvent'>
//       <arg type='u' name='keyval' />
//       <arg type='u' name='keycode' />
//       <arg type='u' name='state' />
//     </signal>
//     <signal name='PanelExtension'>
//       <arg type='v' name='data' />
//     </signal>
//
//     <property name='ContentType' type='(uu)' access='write' />
//     <property name='FocusId' type='(b)' access='read' />
//     <property name='ActiveSurroundingText' type='(b)' access='read' />
//   </interface>
// </node>
#[interface(name = "org.freedesktop.IBus.Engine")]
impl<T: IBusEngine + 'static> Engine<T> {
    async fn process_key_event(
        &mut self,
        #[zbus(signal_context)] sc: SignalContext<'_>,
        keyval: u32,
        keycode: u32,
        state: u32,
    ) -> fdo::Result<bool> {
        self.e.process_key_event(sc, keyval, keycode, state).await
    }

    async fn set_cursor_location(
        &mut self,
        #[zbus(signal_context)] sc: SignalContext<'_>,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    ) -> fdo::Result<()> {
        self.e.set_cursor_location(sc, x, y, w, h).await
    }

    // 忽略
    fn process_hand_writing_event(&mut self, _coordinates: Vec<f64>) -> fdo::Result<()> {
        Ok(())
    }

    // 忽略
    fn cancel_hand_writing(&mut self, _n_strokes: u32) -> fdo::Result<()> {
        Ok(())
    }

    // 忽略
    fn set_capabilities(&mut self, _caps: u32) -> fdo::Result<()> {
        Ok(())
    }

    // 忽略 (用户界面相关)
    fn property_activate(&mut self, _name: String, _state: u32) -> fdo::Result<()> {
        Ok(())
    }

    // 忽略 (用户界面相关)
    fn property_show(&mut self, _name: String) -> fdo::Result<()> {
        Ok(())
    }

    // 忽略 (用户界面相关)
    fn property_hide(&mut self, _name: String) -> fdo::Result<()> {
        Ok(())
    }

    // 忽略 (用户界面相关)
    fn candidate_clicked(&mut self, _index: u32, _button: u32, _state: u32) -> fdo::Result<()> {
        Ok(())
    }

    async fn focus_in(&mut self, #[zbus(signal_context)] sc: SignalContext<'_>) -> fdo::Result<()> {
        self.e.focus_in(sc).await
    }

    fn focus_in_id(&mut self, _object_path: String, _client: String) -> fdo::Result<()> {
        // TODO
        Ok(())
    }

    async fn focus_out(
        &mut self,
        #[zbus(signal_context)] sc: SignalContext<'_>,
    ) -> fdo::Result<()> {
        self.e.focus_out(sc).await
    }

    fn focus_out_id(&mut self, _object_path: String) -> fdo::Result<()> {
        // TODO
        Ok(())
    }

    async fn reset(&mut self, #[zbus(signal_context)] sc: SignalContext<'_>) -> fdo::Result<()> {
        self.e.reset(sc).await
    }

    async fn enable(&mut self, #[zbus(signal_context)] sc: SignalContext<'_>) -> fdo::Result<()> {
        self.e.enable(sc).await
    }

    async fn disable(&mut self, #[zbus(signal_context)] sc: SignalContext<'_>) -> fdo::Result<()> {
        self.e.disable(sc).await
    }

    // 忽略 (用户界面相关)
    fn page_up(&mut self) -> fdo::Result<()> {
        Ok(())
    }

    // 忽略 (用户界面相关)
    fn page_down(&mut self) -> fdo::Result<()> {
        Ok(())
    }

    // 忽略 (用户界面相关)
    fn cursor_up(&mut self) -> fdo::Result<()> {
        Ok(())
    }

    // 忽略 (用户界面相关)
    fn cursor_down(&mut self) -> fdo::Result<()> {
        Ok(())
    }

    // 忽略
    fn set_surrounding_text(
        &mut self,
        _text: Value,
        _cursor_pos: u32,
        _anchor_pos: u32,
    ) -> fdo::Result<()> {
        Ok(())
    }

    // 忽略 (用户界面相关)
    fn panel_extension_received(&mut self, _event: Value) -> fdo::Result<()> {
        Ok(())
    }

    // 忽略 (用户界面相关)
    fn panel_extension_register_keys(&mut self, _data: Value) -> fdo::Result<()> {
        Ok(())
    }

    #[zbus(signal)]
    pub async fn commit_text(sc: &SignalContext<'_>, text: Value<'_>) -> zbus::Result<()>;

    // 忽略 (用户界面相关)
    #[zbus(signal)]
    async fn update_preedit_text(
        sc: &SignalContext<'_>,
        text: Value<'_>,
        cursor_pos: u32,
        visible: bool,
        mode: u32,
    ) -> zbus::Result<()>;

    // 忽略 (用户界面相关)
    #[zbus(signal)]
    async fn update_auxiliary_text(
        sc: &SignalContext<'_>,
        text: Value<'_>,
        visible: bool,
    ) -> zbus::Result<()>;

    // 忽略 (用户界面相关)
    #[zbus(signal)]
    async fn update_lookup_table(
        sc: &SignalContext<'_>,
        table: Value<'_>,
        visible: bool,
    ) -> zbus::Result<()>;

    // 忽略 (用户界面相关)
    #[zbus(signal)]
    async fn register_properties(sc: &SignalContext<'_>, props: Value<'_>) -> zbus::Result<()>;

    // 忽略 (用户界面相关)
    #[zbus(signal)]
    async fn update_property(sc: &SignalContext<'_>, prop: Value<'_>) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn forward_key_event(
        sc: &SignalContext<'_>,
        keyval: u32,
        keycode: u32,
        state: u32,
    ) -> zbus::Result<()>;

    // 忽略 (用户界面相关)
    #[zbus(signal)]
    async fn panel_extension(sc: &SignalContext<'_>, data: Value<'_>) -> zbus::Result<()>;

    #[zbus(property)]
    fn content_type(&self) -> (u32, u32) {
        // TODO
        (0, 0)
    }

    #[zbus(property)]
    fn set_content_type(&mut self, _t: (u32, u32)) -> fdo::Result<()> {
        // TODO
        Ok(())
    }

    #[zbus(property)]
    fn focus_id(&self) -> bool {
        // TODO
        false
    }

    #[zbus(property)]
    fn active_surrounding_text(&self) -> bool {
        // TODO
        false
    }
}

impl<T: IBusEngine + 'static> Engine<T> {
    /// create engine (include ibus init)
    pub async fn new(c: &Connection, e: T) -> Result<String, Box<dyn Error>> {
        // 源文件: `ibus/src/ibusfactory.c`
        // 函数: `ibus_factory_real_create_engine()`
        let object_path = format!("/org/freedesktop/IBus/Engine/{}", 1);

        let o = Engine {
            e,
            _op: object_path.clone(),
        };

        c.object_server().at(object_path.clone(), o).await?;

        info!("创建 engine 成功: {}", object_path);
        Ok(object_path)
    }
}
