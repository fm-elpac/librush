use zbus::zvariant::{Array, Signature, Structure, Value};

// 源文件: `ibus/src/ibustext.c`
// IBusText
//
// variant  struct {
//   string "IBusText"
//   array []
//   string "测试"
//   variant  struct {
//     string "IBusAttrList"
//     array []
//     array []
//   }
// }
//
// (
//   <(
//     'IBusText',
//     @a{sv} {},
//     '测试',
//     <(
//       'IBusAttrList',
//       @a{sv} {},
//       @av []
//     )>
//   )>,
// )

/// `IBusText`: serialize data as ibus format
pub fn make_ibus_text(text: String) -> Value<'static> {
    // 构造 `@a{sv}`
    fn a1() -> Array<'static> {
        let sig = Signature::from_str_unchecked("{sv}");
        Array::new(sig)
    }
    // 构造 `@av`
    fn a2() -> Array<'static> {
        let sig = Signature::from_str_unchecked("v");
        Array::new(sig)
    }

    // 构造内部的 variant  struct
    let st1 = Structure::from(("IBusAttrList", a1(), a2()));

    // 构造外部的 variant  struct
    let st2 = Structure::from(("IBusText", a1(), text, Value::new(st1)));

    Value::new(st2)
}

// ibus 按键定义

// 源文件: `ibus/src/ibustypes.h`

/// 这个标志位表示按键释放 (松开) 消息
pub const IBUS_RELEASE_MASK: u32 = 1 << 30;
/// shift 键
pub const IBUS_SHIFT_MASK: u32 = 1 << 0;
/// ctrl 键
pub const IBUS_CONTROL_MASK: u32 = 1 << 2;
/// Alt 键, Meta_L 键
pub const IBUS_MOD1_MASK: u32 = 1 << 3;
/// Super_L 键, Hyper_L 键
pub const IBUS_MOD4_MASK: u32 = 1 << 6;
/// super (win) 键
pub const IBUS_SUPER_MASK: u32 = 1 << 26;
/// hyper 键
pub const IBUS_HYPER_MASK: u32 = 1 << 27;
/// meta 键
pub const IBUS_META_MASK: u32 = 1 << 28;

// 源文件: `ibus/src/ibuskeysyms.h`

/// 退格键
pub const IBUS_KEY_BACKSPACE: u32 = 0xff08;
/// 回车键
pub const IBUS_KEY_RETURN: u32 = 0xff0d;
/// ESC
pub const IBUS_KEY_ESCAPE: u32 = 0xff1b;
/// 方向键: 左
pub const IBUS_KEY_LEFT: u32 = 0xff51;
/// 方向键: 上
pub const IBUS_KEY_UP: u32 = 0xff52;
/// 方向键: 右
pub const IBUS_KEY_RIGHT: u32 = 0xff53;
/// 方向键: 下
pub const IBUS_KEY_DOWN: u32 = 0xff54;

/// 检查按键消息: 是否为按下按键
pub fn is_keydown(state: u32) -> bool {
    !is_keyup(state)
}

/// 检查按键消息: 是否为松开按键
pub fn is_keyup(state: u32) -> bool {
    (state & IBUS_RELEASE_MASK) != 0
}

/// 检查按键消息: 特殊组合键是否被按下
///
/// 包括: Shift, Ctrl, Alt, Super 等
pub fn is_special_mask(state: u32) -> bool {
    if ((state & IBUS_SHIFT_MASK) != 0)
        || ((state & IBUS_CONTROL_MASK) != 0)
        || ((state & IBUS_MOD1_MASK) != 0)
        || ((state & IBUS_MOD4_MASK) != 0)
        || ((state & IBUS_SUPER_MASK) != 0)
        || ((state & IBUS_HYPER_MASK) != 0)
        || ((state & IBUS_META_MASK) != 0)
    {
        true
    } else {
        false
    }
}
