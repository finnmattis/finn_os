use bitflags::bitflags;
use conquer_once::spin::Lazy;
use spinning_top::Spinlock;
use x86_64::instructions::port::Port;

const ADDRESS_PORT_ADDRESS: u16 = 0x64;
const DATA_PORT_ADDRESS: u16 = 0x60;
const GET_STATUS_BYTE: u8 = 0x20;
const SET_STATUS_BYTE: u8 = 0x60;
const SET_DEFAULTS: u8 = 0xF6;
const ENABLE_PACKET_STREAMING: u8 = 0xF4;

pub static MOUSE: Lazy<Spinlock<Mouse>> = Lazy::new(|| Spinlock::new(Mouse::new()));

fn send_command(
    command_port: &mut Port<u8>,
    data_port: &mut Port<u8>,
    command: u8,
) -> Result<(), &'static str> {
    unsafe {
        command_port.write(0xD4);
        data_port.write(command);
        //0xFA is acknoledgement
        if data_port.read() != 0xFA {
            return Err("mouse did not respond to the command");
        }
        Ok(())
    }
}

pub fn init_mouse() {
    let mut command_port: Port<u8> = Port::new(ADDRESS_PORT_ADDRESS);
    let mut data_port: Port<u8> = Port::new(DATA_PORT_ADDRESS);
    unsafe {
        command_port.write(GET_STATUS_BYTE);
        let status = data_port.read() | 0x02;
        command_port.write(SET_STATUS_BYTE);
        data_port.write(status & 0xDF);

        send_command(&mut command_port, &mut data_port, SET_DEFAULTS).unwrap();
        send_command(&mut command_port, &mut data_port, ENABLE_PACKET_STREAMING).unwrap();
    }
}

bitflags! {
    /// Represents the flags currently set for the mouse.
    #[derive(Default)]
    pub struct MouseFlags: u8 {
        /// Whether or not the left mouse button is pressed.
        const LEFT_BUTTON = 0b0000_0001;

        /// Whether or not the right mouse button is pressed.
        const RIGHT_BUTTON = 0b0000_0010;

        /// Whether or not the middle mouse button is pressed.
        const MIDDLE_BUTTON = 0b0000_0100;

        /// Whether or not the packet is valid or not.
        const ALWAYS_ONE = 0b0000_1000;

        /// Whether or not the x delta is negative.
        const X_SIGN = 0b0001_0000;

        /// Whether or not the y delta is negative.
        const Y_SIGN = 0b0010_0000;

        /// Whether or not the x delta overflowed.
        const X_OVERFLOW = 0b0100_0000;

        /// Whether or not the y delta overflowed.
        const Y_OVERFLOW = 0b1000_0000;
    }
}

/// A snapshot of the mouse flags, x delta and y delta.
#[derive(Debug, Copy, Clone, Default)]
pub struct MouseState {
    flags: MouseFlags,
    x: i16,
    y: i16,
}
#[allow(dead_code)]
impl MouseState {
    /// Returns true if the left mouse button is currently down.
    pub fn left_button_down(&self) -> bool {
        self.flags.contains(MouseFlags::LEFT_BUTTON)
    }

    /// Returns true if the left mouse button is currently up.
    pub fn left_button_up(&self) -> bool {
        !self.flags.contains(MouseFlags::LEFT_BUTTON)
    }

    /// Returns true if the right mouse button is currently down.
    pub fn right_button_down(&self) -> bool {
        self.flags.contains(MouseFlags::RIGHT_BUTTON)
    }

    /// Returns true if the right mouse button is currently up.
    pub fn right_button_up(&self) -> bool {
        !self.flags.contains(MouseFlags::RIGHT_BUTTON)
    }

    /// Returns the x delta of the mouse state.
    pub fn get_x(&self) -> i16 {
        self.x
    }

    /// Returns the y delta of the mouse state.
    pub fn get_y(&self) -> i16 {
        self.y
    }
}

pub struct Mouse {
    current_packet: u8,
    current_state: MouseState,
    completed_state: MouseState,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            current_packet: 0,
            current_state: MouseState::default(),
            completed_state: MouseState::default(),
        }
    }

    pub fn get_coords(&mut self) -> (i16, i16) {
        let res = (self.completed_state.get_x(), self.completed_state.get_y());
        self.completed_state = MouseState::default();
        res
    }

    pub fn process_packet(&mut self, packet: u8) {
        match self.current_packet {
            0 => {
                let flags = MouseFlags::from_bits_truncate(packet);
                if !flags.contains(MouseFlags::ALWAYS_ONE) {
                    return;
                }
                self.current_state.flags = flags;
            }
            1 => self.process_x_movement(packet),
            2 => {
                self.process_y_movement(packet);
                self.completed_state = self.current_state;
            }
            _ => unreachable!(),
        }
        self.current_packet = (self.current_packet + 1) % 3;
    }

    fn process_x_movement(&mut self, packet: u8) {
        if !self.current_state.flags.contains(MouseFlags::X_OVERFLOW) {
            self.current_state.x = if self.current_state.flags.contains(MouseFlags::X_SIGN) {
                self.sign_extend(packet)
            } else {
                packet as i16
            };
        }
    }

    fn process_y_movement(&mut self, packet: u8) {
        if !self.current_state.flags.contains(MouseFlags::Y_OVERFLOW) {
            self.current_state.y = if self.current_state.flags.contains(MouseFlags::Y_SIGN) {
                self.sign_extend(packet)
            } else {
                packet as i16
            };
        }
    }

    fn sign_extend(&self, packet: u8) -> i16 {
        ((packet as u16) | 0xFF00) as i16
    }
}
