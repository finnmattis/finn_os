use super::registers::{
    AttributeControllerIndex, CrtcControllerIndex, GraphicsControllerIndex, SequencerIndex,
};

/// Represents a set of vga registers for a given mode.
#[derive(Debug)]
pub struct VgaConfiguration {
    /// Represents the configuration value for the miscellaneous output register.
    pub miscellaneous_output: u8,
    /// Represents the configuration values for the sequencer registers.
    pub sequencer_registers: &'static [(SequencerIndex, u8)],
    /// Represents the configuration values for the crtc controller registers.
    pub crtc_controller_registers: &'static [(CrtcControllerIndex, u8)],
    /// Represents the configuration values for the graphics controller registers.
    pub graphics_controller_registers: &'static [(GraphicsControllerIndex, u8)],
    /// Represents the configuration values for the attribute controller registers.
    pub attribute_controller_registers: &'static [(AttributeControllerIndex, u8)],
}

/// Register values for Vga mode 320x200x256 Graphics.
pub const CONFIGURATION: VgaConfiguration = VgaConfiguration {
    // Configuration values acquired from https://www.singlix.com/trdos/archive/vga/Graphics%20in%20pmode.pdf
    miscellaneous_output: 0x63,
    sequencer_registers: &[
        (SequencerIndex::SequencerReset, 0x03),
        (SequencerIndex::ClockingMode, 0x01),
        (SequencerIndex::PlaneMask, 0x0F),
        (SequencerIndex::CharacterFont, 0x00),
        (SequencerIndex::MemoryMode, 0x0E),
    ],
    crtc_controller_registers: &[
        (CrtcControllerIndex::HorizontalTotal, 0x5F),
        (CrtcControllerIndex::HorizontalDisplayEnableEnd, 0x4F),
        (CrtcControllerIndex::HorizontalBlankingStart, 0x50),
        (CrtcControllerIndex::HorizontalBlankingEnd, 0x82),
        (CrtcControllerIndex::HorizontalSyncStart, 0x54),
        (CrtcControllerIndex::HorizontalSyncEnd, 0x80),
        (CrtcControllerIndex::VeritcalTotal, 0xBF),
        (CrtcControllerIndex::Overflow, 0x1F),
        (CrtcControllerIndex::PresetRowScan, 0x00),
        (CrtcControllerIndex::MaximumScanLine, 0x41),
        (CrtcControllerIndex::TextCursorStart, 0x00),
        (CrtcControllerIndex::TextCursorEnd, 0x00),
        (CrtcControllerIndex::StartAddressHigh, 0x00),
        (CrtcControllerIndex::StartAddressLow, 0x00),
        (CrtcControllerIndex::TextCursorLocationHigh, 0x00),
        (CrtcControllerIndex::TextCursorLocationLow, 0x00),
        (CrtcControllerIndex::VerticalSyncStart, 0x9C),
        (CrtcControllerIndex::VerticalSyncEnd, 0x0E),
        (CrtcControllerIndex::VerticalDisplayEnableEnd, 0x8F),
        (CrtcControllerIndex::Offset, 0x28),
        (CrtcControllerIndex::UnderlineLocation, 0x40),
        (CrtcControllerIndex::VerticalBlankingStart, 0x96),
        (CrtcControllerIndex::VerticalBlankingEnd, 0xB9),
        (CrtcControllerIndex::ModeControl, 0xA3),
        (CrtcControllerIndex::LineCompare, 0xFF),
    ],
    graphics_controller_registers: &[
        (GraphicsControllerIndex::SetReset, 0x00),
        (GraphicsControllerIndex::EnableSetReset, 0x00),
        (GraphicsControllerIndex::ColorCompare, 0x00),
        (GraphicsControllerIndex::DataRotate, 0x00),
        (GraphicsControllerIndex::ReadPlaneSelect, 0x00),
        (GraphicsControllerIndex::GraphicsMode, 0x40),
        (GraphicsControllerIndex::Miscellaneous, 0x05),
        (GraphicsControllerIndex::ColorDontCare, 0x0F),
        (GraphicsControllerIndex::BitMask, 0xFF),
    ],
    attribute_controller_registers: &[
        (AttributeControllerIndex::PaletteRegister0, 0x00),
        (AttributeControllerIndex::PaletteRegister1, 0x01),
        (AttributeControllerIndex::PaletteRegister2, 0x02),
        (AttributeControllerIndex::PaletteRegister3, 0x03),
        (AttributeControllerIndex::PaletteRegister4, 0x04),
        (AttributeControllerIndex::PaletteRegister5, 0x05),
        (AttributeControllerIndex::PaletteRegister6, 0x06),
        (AttributeControllerIndex::PaletteRegister7, 0x07),
        (AttributeControllerIndex::PaletteRegister8, 0x08),
        (AttributeControllerIndex::PaletteRegister9, 0x09),
        (AttributeControllerIndex::PaletteRegisterA, 0x0A),
        (AttributeControllerIndex::PaletteRegisterB, 0x0B),
        (AttributeControllerIndex::PaletteRegisterC, 0x0C),
        (AttributeControllerIndex::PaletteRegisterD, 0x0D),
        (AttributeControllerIndex::PaletteRegisterE, 0x0E),
        (AttributeControllerIndex::PaletteRegisterF, 0x0F),
        (AttributeControllerIndex::ModeControl, 0x41),
        (AttributeControllerIndex::OverscanColor, 0x00),
        (AttributeControllerIndex::MemoryPlaneEnable, 0x0F),
        (AttributeControllerIndex::HorizontalPixelPanning, 0x00),
        (AttributeControllerIndex::ColorSelect, 0x00),
    ],
};
