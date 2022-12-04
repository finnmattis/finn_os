use super::{
    configuration::{VgaConfiguration, CONFIGURATION},
    registers::{
        AttributeControllerRegisters, ColorPaletteRegisters, CrtcControllerIndex,
        CrtcControllerRegisters, EmulationMode, GeneralRegisters, GraphicsControllerRegisters,
        SequencerRegisters,
    },
};
use crate::graphics::{
    colors::DEFAULT_PALETTE,
    lines::{Bresenham, Point},
};
use alloc::boxed::Box;
use conquer_once::spin::Lazy;
use core::ptr::copy_nonoverlapping;
use font8x8::UnicodeFonts;
use lazy_static::lazy_static;
use spinning_top::Spinlock;

/// The starting address for graphics modes.
const FRAME_BUFFER: *mut u8 = 0xa0000 as *mut u8;

lazy_static! {
    static ref DOUBLE_BUFFER: Box<[u8; 320 * 200]> = Box::new([0; 320 * 200]);
}

const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const SIZE: usize = WIDTH * HEIGHT;

/// Provides mutable access to the vga graphics card.
pub static VGA: Lazy<Spinlock<Vga>> = Lazy::new(|| Spinlock::new(Vga::new()));

/// Represents a vga graphics card with it's common registers.
pub struct Vga {
    /// Represents the general registers on vga hardware.
    pub general_registers: GeneralRegisters,
    /// Represents the sequencer registers on vga hardware.
    pub sequencer_registers: SequencerRegisters,
    /// Represents the graphics controller registers on vga hardware.
    pub graphics_controller_registers: GraphicsControllerRegisters,
    /// Represents the attribute controller registers on vga hardware.
    pub attribute_controller_registers: AttributeControllerRegisters,
    /// Represents the crtc controller registers on vga hardware.
    pub crtc_controller_registers: CrtcControllerRegisters,
    /// Represents the color palette registers on vga hardware.
    pub color_palette_registers: ColorPaletteRegisters,
}

impl Vga {
    fn new() -> Vga {
        Vga {
            general_registers: GeneralRegisters::new(),
            sequencer_registers: SequencerRegisters::new(),
            graphics_controller_registers: GraphicsControllerRegisters::new(),
            attribute_controller_registers: AttributeControllerRegisters::new(),
            crtc_controller_registers: CrtcControllerRegisters::new(),
            color_palette_registers: ColorPaletteRegisters::new(),
        }
    }

    /// Sets the video card to Mode 640x480x16 and clears the screen.
    pub fn setup(&mut self) {
        self.set_registers(&CONFIGURATION);
        // Some bios mess up the palette when switching modes,
        // so explicitly set it.
        self.color_palette_registers.load_palette(&DEFAULT_PALETTE);
        self.clear_screen(0);
        self.swap_buffers();
    }

    pub fn get_buffer(&self) -> *mut u8 {
        DOUBLE_BUFFER.as_ptr() as *mut u8
    }

    /// Returns the current `EmulationMode` as determined by the miscellaneous output register.
    pub fn get_emulation_mode(&mut self) -> EmulationMode {
        EmulationMode::from(self.general_registers.read_msr() & 0x1)
    }

    /// Unlocks the CRTC registers by setting bit 7 to 0 `(value & 0x7F)`.
    ///
    /// `Protect Registers [0:7]`: Note that the ability to write to Bit 4 of the Overflow Register (CR07)
    /// is not affected by this bit (i.e., bit 4 of the Overflow Register is always writeable).
    ///
    /// 0 = Enable writes to registers `CR[00:07]`
    ///
    /// 1 = Disable writes to registers `CR[00:07]`
    fn unlock_crtc_registers(&mut self, emulation_mode: EmulationMode) {
        // Setting bit 7 to 1 used to be required for `VGA`, but says it's
        // ignored in modern hardware. Setting it to 1 just to be safe for older
        // hardware. More information can be found here
        // https://01.org/sites/default/files/documentation/intel-gfx-prm-osrc-hsw-display.pdf
        // under `CR03 - Horizontal Blanking End Register`.
        let horizontal_blanking_end = self
            .crtc_controller_registers
            .read(emulation_mode, CrtcControllerIndex::HorizontalBlankingEnd);
        self.crtc_controller_registers.write(
            emulation_mode,
            CrtcControllerIndex::HorizontalBlankingEnd,
            horizontal_blanking_end | 0x80,
        );

        let vertical_sync_end = self
            .crtc_controller_registers
            .read(emulation_mode, CrtcControllerIndex::VerticalSyncEnd);
        self.crtc_controller_registers.write(
            emulation_mode,
            CrtcControllerIndex::VerticalSyncEnd,
            vertical_sync_end & 0x7F,
        );
    }

    fn set_registers(&mut self, configuration: &VgaConfiguration) {
        let emulation_mode = self.get_emulation_mode();

        // Set miscellaneous output
        self.general_registers
            .write_msr(configuration.miscellaneous_output);

        // Set the sequencer registers.
        for (index, value) in configuration.sequencer_registers {
            self.sequencer_registers.write(*index, *value);
        }

        // Unlock the crtc registers.
        self.unlock_crtc_registers(emulation_mode);

        // Set the crtc registers.
        for (index, value) in configuration.crtc_controller_registers {
            self.crtc_controller_registers
                .write(emulation_mode, *index, *value);
        }

        // Set the grx registers.
        for (index, value) in configuration.graphics_controller_registers {
            self.graphics_controller_registers.write(*index, *value);
        }

        // Blank the screen so the palette registers are unlocked.
        self.attribute_controller_registers
            .blank_screen(emulation_mode);

        // Set the arx registers.
        for (index, value) in configuration.attribute_controller_registers {
            self.attribute_controller_registers
                .write(emulation_mode, *index, *value);
        }

        // Unblank the screen so the palette registers are locked.
        self.attribute_controller_registers
            .unblank_screen(emulation_mode);
    }

    pub fn swap_buffers(&self) {
        unsafe {
            copy_nonoverlapping(self.get_buffer(), FRAME_BUFFER, SIZE);
        }
    }

    pub fn clear_screen(&self, color: u8) {
        unsafe {
            self.get_buffer().write_bytes(color, SIZE);
        }
    }

    #[inline]
    fn _set_pixel(&self, x: usize, y: usize, color: u8) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }

        let offset = (y * WIDTH) + x;
        unsafe {
            self.get_buffer().add(offset).write_volatile(color);
        }
    }

    fn draw_character(&mut self, x: usize, y: usize, character: char, color: u8) {
        let character = match font8x8::BASIC_FONTS.get(character) {
            Some(character) => character,
            // Default to a filled block if the character isn't found
            None => font8x8::unicode::BLOCK_UNICODE[8].byte_array(),
        };

        for (row, byte) in character.iter().enumerate() {
            for bit in 0..8 {
                match *byte & 1 << bit {
                    0 => (),
                    _ => self._set_pixel(x + bit, y + row, color),
                }
            }
        }
    }

    #[inline]
    pub fn draw_line(&mut self, start: Point<isize>, end: Point<isize>, color: u8) {
        for (x, y) in Bresenham::new(start, end) {
            self._set_pixel(x as usize, y as usize, color);
        }
    }

    pub fn draw_triangle(
        &mut self,
        v1: Point<isize>,
        v2: Point<isize>,
        v3: Point<isize>,
        color: u8,
    ) {
        self.draw_line(v1, v2, color);
        self.draw_line(v2, v3, color);
        self.draw_line(v3, v1, color);
    }

    fn fill_bottom_triangle(
        &mut self,
        v1: Point<isize>,
        v2: Point<isize>,
        v3: Point<isize>,
        color: u8,
    ) {
        let (x1, y1) = v1;
        let (x2, y2) = v2;
        let (x3, y3) = v3;

        let invslope1 = (x2 - x1) as f32 / (y2 - y1) as f32;
        let invslope2 = (x3 - x1) as f32 / (y3 - y1) as f32;

        let mut curx1: f32 = x1 as f32;
        let mut curx2: f32 = x1 as f32;

        for i in y1..y2 {
            self.draw_line((curx1 as isize, i), (curx2 as isize, i), color);
            curx1 += invslope1;
            curx2 += invslope2;
        }
    }

    fn fill_top_triangle(
        &mut self,
        v1: Point<isize>,
        v2: Point<isize>,
        v3: Point<isize>,
        color: u8,
    ) {
        let (x1, y1) = v1;
        let (x2, y2) = v2;
        let (x3, y3) = v3;

        let invslope1 = (x3 - x1) as f32 / (y3 - y1) as f32;
        let invslope2 = (x3 - x2) as f32 / (y3 - y2) as f32;

        let mut curx1: f32 = x3 as f32;
        let mut curx2: f32 = x3 as f32;

        for i in (y1..=y3).rev() {
            self.draw_line((curx1 as isize, i), (curx2 as isize, i), color);
            curx1 -= invslope1;
            curx2 -= invslope2;
        }
    }

    pub fn fill_triangle(
        &mut self,
        inv1: Point<isize>,
        inv2: Point<isize>,
        inv3: Point<isize>,
        color: u8,
    ) {
        //Sort inv1, inv2, inv3 by y coordinate
        let mut vertices = [inv1, inv2, inv3];
        vertices.sort_by(|a, b| a.1.cmp(&b.1));
        let [v1, v2, v3] = vertices;
        let (x1, y1) = v1;
        let (_x2, y2) = v2;
        let (x3, y3) = v3;

        if y2 == y3 {
            // Since verticies are sorted - this means bottom two must be parellell
            self.fill_bottom_triangle(v1, v2, v3, color);
            //
        } else if y1 == y2 {
            // Since verticies are sorted - this means top two must be parellell
            self.fill_top_triangle(v1, v2, v3, color);
        } else {
            //general case - split the triangle in a topflat and bottom-flat one
            //v4 has the same y coordinate as v2
            //Find the x coordinate of v4
            // x4 / x3 = y4 / y3
            // x4 / x3 = y2 / y3
            // (x4 - x1)/(x3-x1)=(y2-y1)(y3-y1)
            // x4 = x1 + ((y2)/(y3-y1)) * (x3-x1)
            let x = (x1 as f32 + ((y2 - y1) as f32 / (y3 - y1) as f32) * (x3 - x1) as f32) as isize;

            let v4 = (x, y2);
            self.fill_bottom_triangle(v1, v2, v4, color);
            self.fill_top_triangle(v2, v4, v3, color);
        }
    }
}
