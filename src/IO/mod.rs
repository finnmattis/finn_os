mod keyboard;
mod mouse;
mod serial;

pub use keyboard::{
    add_scancode, get_key_ev, KeyCode, KeyEvent, KeyState, Keyboard, SCANCODE_QUEUE,
};

pub use mouse::init_mouse;
pub use mouse::MOUSE;
pub use serial::_print;
