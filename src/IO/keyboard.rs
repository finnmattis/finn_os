use crate::serial_println;
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

//use OnceCell over lazy_static bc OnceCell type has the advantage that we can ensure that the initialization does not happen in the interrupt handler, thus preventing the interrupt handler from performing a heap allocation
pub static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

/// called by the keyboard interrupt handler - must not block or allocate.
pub fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if queue.push(scancode).is_err() {
            serial_println!("WARNING: scancode queue full; dropping keyboard input");
        }
    } else {
        serial_println!("WARNING: scancode queue uninitialized");
    }
}

#[derive(Debug, PartialEq)]
pub enum KeyCode {
    AltLeft = 0,
    AltRight = 1,
    ArrowDown = 2,
    ArrowLeft = 3,
    ArrowRight = 4,
    ArrowUp = 5,
    BackSlash = 6,
    Backspace = 7,
    BackTick = 8,
    BracketSquareLeft = 9,
    BracketSquareRight = 10,
    Break = 11,
    CapsLock = 12,
    Comma = 13,
    ControlLeft = 14,
    ControlRight = 15,
    Delete = 16,
    End = 17,
    Enter = 18,
    Escape = 19,
    Equals = 20,
    F1 = 21,
    F2 = 22,
    F3 = 23,
    F4 = 24,
    F5 = 26,
    F6 = 27,
    F7 = 28,
    F8 = 29,
    F9 = 30,
    F10 = 31,
    F11 = 32,
    F12 = 33,
    Fullstop = 34,
    Home = 36,
    Insert = 37,
    Key1 = 38,
    Key2 = 39,
    Key3 = 40,
    Key4 = 41,
    Key5 = 42,
    Key6 = 43,
    Key7 = 44,
    Key8 = 46,
    Key9 = 47,
    Key0 = 48,
    Menus = 49,
    Minus = 50,
    Numpad0 = 51,
    Numpad1 = 52,
    Numpad2 = 53,
    Numpad3 = 54,
    Numpad4 = 56,
    Numpad5 = 57,
    Numpad6 = 58,
    Numpad7 = 59,
    Numpad8 = 60,
    Numpad9 = 61,
    NumpadEnter = 62,
    NumpadLock = 63,
    NumpadSlash = 64,
    NumpadStar = 66,
    NumpadMinus = 67,
    NumpadPeriod = 68,
    NumpadPlus = 69,
    PageDown = 70,
    PageUp = 71,
    PauseBreak = 72,
    PrintScreen = 73,
    ScrollLock = 74,
    SemiColon = 76,
    ShiftLeft = 77,
    ShiftRight = 78,
    Slash = 79,
    Spacebar = 80,
    SysReq = 81,
    Tab = 82,
    Quote = 83,
    WindowsLeft = 84,
    WindowsRight = 86,
    A = 87,
    B = 88,
    C = 89,
    D = 90,
    E = 91,
    F = 92,
    G = 93,
    H = 94,
    I = 96,
    J = 97,
    K = 98,
    L = 99,
    M = 100,
    N = 101,
    O = 102,
    P = 103,
    Q = 104,
    R = 106,
    S = 107,
    T = 108,
    U = 109,
    V = 110,
    W = 111,
    X = 112,
    Y = 113,
    Z = 114,
}
#[derive(Debug, PartialEq)]
pub enum KeyState {
    Up,
    Down,
}
#[derive(Debug)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub state: KeyState,
}

//KeyEvent useful for processing input
impl KeyEvent {
    pub fn new(code: KeyCode, state: KeyState) -> Self {
        Self { code, state }
    }
}
//DecodedKey useful for displaying
pub enum DecodedKey {
    RawKey(KeyCode),
    Unicode(char),
}

pub fn get_key_ev(code: u8) -> Result<KeyEvent, ()> {
    match code {
        0x80..=0xFF => {
            // Release codes
            Ok(KeyEvent::new(get_key_code(code - 0x80)?, KeyState::Up))
        }
        _ => {
            // Normal codes
            Ok(KeyEvent::new(get_key_code(code)?, KeyState::Down))
        }
    }
}

fn get_key_code(code: u8) -> Result<KeyCode, ()> {
    //uses scancode set 1 (IMB XT)
    //values from https://wiki.osdev.org/PS/2_Keyboard#Scan_Code_Set_1
    match code {
        0x01 => Ok(KeyCode::Escape),             // 01
        0x02 => Ok(KeyCode::Key1),               // 02
        0x03 => Ok(KeyCode::Key2),               // 03
        0x04 => Ok(KeyCode::Key3),               // 04
        0x05 => Ok(KeyCode::Key4),               // 05
        0x06 => Ok(KeyCode::Key5),               // 06
        0x07 => Ok(KeyCode::Key6),               // 07
        0x08 => Ok(KeyCode::Key7),               // 08
        0x09 => Ok(KeyCode::Key8),               // 09
        0x0A => Ok(KeyCode::Key9),               // 0A
        0x0B => Ok(KeyCode::Key0),               // 0B
        0x0C => Ok(KeyCode::Minus),              // 0C
        0x0D => Ok(KeyCode::Equals),             // 0D
        0x0E => Ok(KeyCode::Backspace),          // 0E
        0x0F => Ok(KeyCode::Tab),                // 0F
        0x10 => Ok(KeyCode::Q),                  // 10
        0x11 => Ok(KeyCode::W),                  // 11
        0x12 => Ok(KeyCode::E),                  // 12
        0x13 => Ok(KeyCode::R),                  // 13
        0x14 => Ok(KeyCode::T),                  // 14
        0x15 => Ok(KeyCode::Y),                  // 15
        0x16 => Ok(KeyCode::U),                  // 16
        0x17 => Ok(KeyCode::I),                  // 17
        0x18 => Ok(KeyCode::O),                  // 18
        0x19 => Ok(KeyCode::P),                  // 19
        0x1A => Ok(KeyCode::BracketSquareLeft),  // 1A
        0x1B => Ok(KeyCode::BracketSquareRight), // 1B
        0x1C => Ok(KeyCode::Enter),              // 1C
        0x1D => Ok(KeyCode::ControlLeft),        // 1D
        0x1E => Ok(KeyCode::A),                  // 1E
        0x1F => Ok(KeyCode::S),                  // 1F
        0x20 => Ok(KeyCode::D),                  // 20
        0x21 => Ok(KeyCode::F),                  // 21
        0x22 => Ok(KeyCode::G),                  // 22
        0x23 => Ok(KeyCode::H),                  // 23
        0x24 => Ok(KeyCode::J),                  // 24
        0x25 => Ok(KeyCode::K),                  // 25
        0x26 => Ok(KeyCode::L),                  // 26
        0x27 => Ok(KeyCode::SemiColon),          // 27
        0x28 => Ok(KeyCode::Quote),              // 28
        0x29 => Ok(KeyCode::BackTick),           // 29
        0x2A => Ok(KeyCode::ShiftLeft),          // 2A
        0x2B => Ok(KeyCode::BackSlash),          // 2B
        0x2C => Ok(KeyCode::Z),                  // 2C
        0x2D => Ok(KeyCode::X),                  // 2D
        0x2E => Ok(KeyCode::C),                  // 2E
        0x2F => Ok(KeyCode::V),                  // 2F
        0x30 => Ok(KeyCode::B),                  // 30
        0x31 => Ok(KeyCode::N),                  // 31
        0x32 => Ok(KeyCode::M),                  // 32
        0x33 => Ok(KeyCode::Comma),              // 33
        0x34 => Ok(KeyCode::Fullstop),           // 34
        0x35 => Ok(KeyCode::Slash),              // 35
        0x36 => Ok(KeyCode::ShiftRight),         // 36
        0x37 => Ok(KeyCode::NumpadStar),         // 37
        0x38 => Ok(KeyCode::AltLeft),            // 38
        0x39 => Ok(KeyCode::Spacebar),           // 39
        0x3A => Ok(KeyCode::CapsLock),           // 3A
        0x3B => Ok(KeyCode::F1),                 // 3B
        0x3C => Ok(KeyCode::F2),                 // 3C
        0x3D => Ok(KeyCode::F3),                 // 3D
        0x3E => Ok(KeyCode::F4),                 // 3E
        0x3F => Ok(KeyCode::F5),                 // 3F
        0x40 => Ok(KeyCode::F6),                 // 40
        0x41 => Ok(KeyCode::F7),                 // 41
        0x42 => Ok(KeyCode::F8),                 // 42
        0x43 => Ok(KeyCode::F9),                 // 43
        0x44 => Ok(KeyCode::F10),                // 44
        0x45 => Ok(KeyCode::NumpadLock),         // 45
        0x46 => Ok(KeyCode::ScrollLock),         // 46
        0x47 => Ok(KeyCode::Numpad7),            // 47
        0x48 => Ok(KeyCode::Numpad8),            // 48
        0x49 => Ok(KeyCode::Numpad9),            // 49
        0x4A => Ok(KeyCode::NumpadMinus),        // 4A
        0x4B => Ok(KeyCode::Numpad4),            // 4B
        0x4C => Ok(KeyCode::Numpad5),            // 4C
        0x4D => Ok(KeyCode::Numpad6),            // 4D
        0x4E => Ok(KeyCode::NumpadPlus),         // 4E
        0x4F => Ok(KeyCode::Numpad1),            // 4F
        0x50 => Ok(KeyCode::Numpad2),            // 50
        0x51 => Ok(KeyCode::Numpad3),            // 51
        0x52 => Ok(KeyCode::Numpad0),            // 52
        0x53 => Ok(KeyCode::NumpadPeriod),       // 53
        //0x54
        //0x55
        // 0x56
        0x57 => Ok(KeyCode::F11), // 57
        0x58 => Ok(KeyCode::F12), // 58
        0x81..=0xD8 => Ok(get_key_code(code - 0x80)?),
        _ => Err(()),
    }
}

pub struct Keyboard {
    lshift: bool,
    rshift: bool,
    lctrl: bool,
    rctrl: bool,
    numlock: bool,
    capslock: bool,
    alt_gr: bool,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            lshift: false,
            rshift: false,
            lctrl: false,
            rctrl: false,
            numlock: false,
            capslock: false,
            alt_gr: false,
        }
    }

    const fn is_shifted(&self) -> bool {
        self.lshift | self.rshift
    }

    const fn is_ctrl(&self) -> bool {
        self.lctrl | self.rctrl
    }

    const fn is_caps(&self) -> bool {
        (self.lshift | self.rshift) ^ self.capslock
    }

    pub fn process_key_ev(&mut self, ev: KeyEvent) -> Option<DecodedKey> {
        //Modifier Keys update the field of the struct - every other key returns DecodedKey from map_keycode function
        match ev {
            KeyEvent {
                code: KeyCode::ShiftLeft,
                state: KeyState::Down,
            } => {
                self.lshift = true;
                None
            }
            KeyEvent {
                code: KeyCode::ShiftRight,
                state: KeyState::Down,
            } => {
                self.rshift = true;
                None
            }
            KeyEvent {
                code: KeyCode::ShiftLeft,
                state: KeyState::Up,
            } => {
                self.lshift = false;
                None
            }
            KeyEvent {
                code: KeyCode::ShiftRight,
                state: KeyState::Up,
            } => {
                self.rshift = false;
                None
            }
            KeyEvent {
                code: KeyCode::CapsLock,
                state: KeyState::Down,
            } => {
                self.capslock = !self.capslock;
                None
            }
            KeyEvent {
                code: KeyCode::NumpadLock,
                state: KeyState::Down,
            } => {
                self.numlock = !self.numlock;
                None
            }
            KeyEvent {
                code: KeyCode::ControlLeft,
                state: KeyState::Down,
            } => {
                self.lctrl = true;
                None
            }
            KeyEvent {
                code: KeyCode::ControlLeft,
                state: KeyState::Up,
            } => {
                self.lctrl = false;
                None
            }
            KeyEvent {
                code: KeyCode::ControlRight,
                state: KeyState::Down,
            } => {
                self.rctrl = true;
                None
            }
            KeyEvent {
                code: KeyCode::ControlRight,
                state: KeyState::Up,
            } => {
                self.rctrl = false;
                None
            }
            KeyEvent {
                code: KeyCode::AltRight,
                state: KeyState::Down,
            } => {
                self.alt_gr = true;
                None
            }
            KeyEvent {
                code: KeyCode::AltRight,
                state: KeyState::Up,
            } => {
                self.alt_gr = false;
                None
            }
            KeyEvent {
                code: c,
                state: KeyState::Down,
            } => Some(self.map_keycode(c)),
            _ => None,
        }
    }

    fn map_keycode(&self, code: KeyCode) -> DecodedKey {
        //US 104
        match code {
            KeyCode::BackTick => {
                if self.is_shifted() {
                    DecodedKey::Unicode('~')
                } else {
                    DecodedKey::Unicode('`')
                }
            }
            KeyCode::Escape => DecodedKey::Unicode(0x1B.into()),
            KeyCode::Key1 => {
                if self.is_shifted() {
                    DecodedKey::Unicode('!')
                } else {
                    DecodedKey::Unicode('1')
                }
            }
            KeyCode::Key2 => {
                if self.is_shifted() {
                    DecodedKey::Unicode('@')
                } else {
                    DecodedKey::Unicode('2')
                }
            }
            KeyCode::Key3 => {
                if self.is_shifted() {
                    DecodedKey::Unicode('#')
                } else {
                    DecodedKey::Unicode('3')
                }
            }
            KeyCode::Key4 => {
                if self.is_shifted() {
                    DecodedKey::Unicode('$')
                } else {
                    DecodedKey::Unicode('4')
                }
            }
            KeyCode::Key5 => {
                if self.is_shifted() {
                    DecodedKey::Unicode('%')
                } else {
                    DecodedKey::Unicode('5')
                }
            }
            KeyCode::Key6 => {
                if self.is_shifted() {
                    DecodedKey::Unicode('^')
                } else {
                    DecodedKey::Unicode('6')
                }
            }
            KeyCode::Key7 => {
                if self.is_shifted() {
                    DecodedKey::Unicode('&')
                } else {
                    DecodedKey::Unicode('7')
                }
            }
            KeyCode::Key8 => {
                if self.is_shifted() {
                    DecodedKey::Unicode('*')
                } else {
                    DecodedKey::Unicode('8')
                }
            }
            KeyCode::Key9 => {
                if self.is_shifted() {
                    DecodedKey::Unicode('(')
                } else {
                    DecodedKey::Unicode('9')
                }
            }
            KeyCode::Key0 => {
                if self.is_shifted() {
                    DecodedKey::Unicode(')')
                } else {
                    DecodedKey::Unicode('0')
                }
            }
            KeyCode::Minus => {
                if self.is_shifted() {
                    DecodedKey::Unicode('_')
                } else {
                    DecodedKey::Unicode('-')
                }
            }
            KeyCode::Equals => {
                if self.is_shifted() {
                    DecodedKey::Unicode('+')
                } else {
                    DecodedKey::Unicode('=')
                }
            }
            KeyCode::Backspace => DecodedKey::Unicode(0x08.into()),
            KeyCode::Tab => DecodedKey::Unicode(0x09.into()),
            KeyCode::Q => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0011}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('Q')
                } else {
                    DecodedKey::Unicode('q')
                }
            }
            KeyCode::W => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0017}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('W')
                } else {
                    DecodedKey::Unicode('w')
                }
            }
            KeyCode::E => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0005}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('E')
                } else {
                    DecodedKey::Unicode('e')
                }
            }
            KeyCode::R => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0012}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('R')
                } else {
                    DecodedKey::Unicode('r')
                }
            }
            KeyCode::T => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0014}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('T')
                } else {
                    DecodedKey::Unicode('t')
                }
            }
            KeyCode::Y => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0019}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('Y')
                } else {
                    DecodedKey::Unicode('y')
                }
            }
            KeyCode::U => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0015}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('U')
                } else {
                    DecodedKey::Unicode('u')
                }
            }
            KeyCode::I => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0009}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('I')
                } else {
                    DecodedKey::Unicode('i')
                }
            }
            KeyCode::O => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{000F}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('O')
                } else {
                    DecodedKey::Unicode('o')
                }
            }
            KeyCode::P => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0010}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('P')
                } else {
                    DecodedKey::Unicode('p')
                }
            }
            KeyCode::BracketSquareLeft => {
                if self.is_shifted() {
                    DecodedKey::Unicode('{')
                } else {
                    DecodedKey::Unicode('[')
                }
            }
            KeyCode::BracketSquareRight => {
                if self.is_shifted() {
                    DecodedKey::Unicode('}')
                } else {
                    DecodedKey::Unicode(']')
                }
            }
            KeyCode::BackSlash => {
                if self.is_shifted() {
                    DecodedKey::Unicode('|')
                } else {
                    DecodedKey::Unicode('\\')
                }
            }
            KeyCode::A => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0001}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('A')
                } else {
                    DecodedKey::Unicode('a')
                }
            }
            KeyCode::S => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0013}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('S')
                } else {
                    DecodedKey::Unicode('s')
                }
            }
            KeyCode::D => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0004}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('D')
                } else {
                    DecodedKey::Unicode('d')
                }
            }
            KeyCode::F => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0006}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('F')
                } else {
                    DecodedKey::Unicode('f')
                }
            }
            KeyCode::G => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0007}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('G')
                } else {
                    DecodedKey::Unicode('g')
                }
            }
            KeyCode::H => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0008}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('H')
                } else {
                    DecodedKey::Unicode('h')
                }
            }
            KeyCode::J => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{000A}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('J')
                } else {
                    DecodedKey::Unicode('j')
                }
            }
            KeyCode::K => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{000B}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('K')
                } else {
                    DecodedKey::Unicode('k')
                }
            }
            KeyCode::L => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{000C}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('L')
                } else {
                    DecodedKey::Unicode('l')
                }
            }
            KeyCode::SemiColon => {
                if self.is_shifted() {
                    DecodedKey::Unicode(':')
                } else {
                    DecodedKey::Unicode(';')
                }
            }
            KeyCode::Quote => {
                if self.is_shifted() {
                    DecodedKey::Unicode('"')
                } else {
                    DecodedKey::Unicode('\'')
                }
            }
            // Enter gives LF, not CRLF or CR
            KeyCode::Enter => DecodedKey::Unicode(10.into()),
            KeyCode::Z => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{001A}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('Z')
                } else {
                    DecodedKey::Unicode('z')
                }
            }
            KeyCode::X => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0018}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('X')
                } else {
                    DecodedKey::Unicode('x')
                }
            }
            KeyCode::C => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0003}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('C')
                } else {
                    DecodedKey::Unicode('c')
                }
            }
            KeyCode::V => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0016}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('V')
                } else {
                    DecodedKey::Unicode('v')
                }
            }
            KeyCode::B => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{0002}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('B')
                } else {
                    DecodedKey::Unicode('b')
                }
            }
            KeyCode::N => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{000E}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('N')
                } else {
                    DecodedKey::Unicode('n')
                }
            }
            KeyCode::M => {
                if self.is_ctrl() {
                    DecodedKey::Unicode('\u{000D}')
                } else if self.is_caps() {
                    DecodedKey::Unicode('M')
                } else {
                    DecodedKey::Unicode('m')
                }
            }
            KeyCode::Comma => {
                if self.is_shifted() {
                    DecodedKey::Unicode('<')
                } else {
                    DecodedKey::Unicode(',')
                }
            }
            KeyCode::Fullstop => {
                if self.is_shifted() {
                    DecodedKey::Unicode('>')
                } else {
                    DecodedKey::Unicode('.')
                }
            }
            KeyCode::Slash => {
                if self.is_shifted() {
                    DecodedKey::Unicode('?')
                } else {
                    DecodedKey::Unicode('/')
                }
            }
            KeyCode::Spacebar => DecodedKey::Unicode(' '),
            KeyCode::Delete => DecodedKey::Unicode(127.into()),
            KeyCode::NumpadSlash => DecodedKey::Unicode('/'),
            KeyCode::NumpadStar => DecodedKey::Unicode('*'),
            KeyCode::NumpadMinus => DecodedKey::Unicode('-'),
            KeyCode::Numpad7 => {
                if self.numlock {
                    DecodedKey::Unicode('7')
                } else {
                    DecodedKey::RawKey(KeyCode::Home)
                }
            }
            KeyCode::Numpad8 => {
                if self.numlock {
                    DecodedKey::Unicode('8')
                } else {
                    DecodedKey::RawKey(KeyCode::ArrowUp)
                }
            }
            KeyCode::Numpad9 => {
                if self.numlock {
                    DecodedKey::Unicode('9')
                } else {
                    DecodedKey::RawKey(KeyCode::PageUp)
                }
            }
            KeyCode::NumpadPlus => DecodedKey::Unicode('+'),
            KeyCode::Numpad4 => {
                if self.numlock {
                    DecodedKey::Unicode('4')
                } else {
                    DecodedKey::RawKey(KeyCode::ArrowLeft)
                }
            }
            KeyCode::Numpad5 => DecodedKey::Unicode('5'),
            KeyCode::Numpad6 => {
                if self.numlock {
                    DecodedKey::Unicode('6')
                } else {
                    DecodedKey::RawKey(KeyCode::ArrowRight)
                }
            }
            KeyCode::Numpad1 => {
                if self.numlock {
                    DecodedKey::Unicode('1')
                } else {
                    DecodedKey::RawKey(KeyCode::End)
                }
            }
            KeyCode::Numpad2 => {
                if self.numlock {
                    DecodedKey::Unicode('2')
                } else {
                    DecodedKey::RawKey(KeyCode::ArrowDown)
                }
            }
            KeyCode::Numpad3 => {
                if self.numlock {
                    DecodedKey::Unicode('3')
                } else {
                    DecodedKey::RawKey(KeyCode::PageDown)
                }
            }
            KeyCode::Numpad0 => {
                if self.numlock {
                    DecodedKey::Unicode('0')
                } else {
                    DecodedKey::RawKey(KeyCode::Insert)
                }
            }
            KeyCode::NumpadPeriod => {
                if self.numlock {
                    DecodedKey::Unicode('.')
                } else {
                    DecodedKey::Unicode(127.into())
                }
            }
            KeyCode::NumpadEnter => DecodedKey::Unicode(10.into()),
            k => DecodedKey::RawKey(k),
        }
    }
}
