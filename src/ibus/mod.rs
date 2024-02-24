//! ibus D-Bus interface api-bindings
//!
//! <https://ibus.github.io/docs/ibus-1.5/index.html>
mod addr;
mod bus;
mod engine;
mod error;
mod factory;
mod ibus_serde;
mod init;

pub use addr::get_ibus_addr;
pub use bus::IBus;
pub use engine::{Engine, IBusEngine};
pub use error::IBusErr;
pub use factory::IBusFactory;
pub use ibus_serde::{
    is_keydown, is_keyup, is_special_mask, make_ibus_text, IBUS_KEY_BACKSPACE, IBUS_KEY_DELETE,
    IBUS_KEY_DOWN, IBUS_KEY_ESCAPE, IBUS_KEY_LEFT, IBUS_KEY_RETURN, IBUS_KEY_RIGHT, IBUS_KEY_UP,
    IBUS_RELEASE_MASK,
};
