mod keyboard;
mod mouse;

pub use keyboard::{
    add_scancode, get_key_ev, KeyCode, KeyEvent, KeyState, Keyboard, SCANCODE_QUEUE,
};

pub use mouse::init_mouse;
pub use mouse::MOUSE;
