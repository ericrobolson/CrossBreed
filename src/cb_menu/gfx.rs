use super::*;

/// Struct that represents a color
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        return Self { r: r, b: b, g: g };
    }

    pub fn black() -> Self {
        return Self::new(0, 0, 0);
    }
}

/// A simplified way to keep consistent pallates.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Palette {
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,
    pub quaternary: Color,
    pub background: Color,
    pub accent: Color,
}

impl Palette {
    pub fn new() -> Self {
        return Self {
            primary: Color::new(222, 113, 25),
            secondary: Color::new(222, 227, 226),
            tertiary: Color::new(17, 105, 121),
            quaternary: Color::new(0, 0, 0),
            background: Color::new(255, 255, 255),
            accent: Color::new(24, 176, 176),
        };
    }
}

pub enum CbMenuDrawVirtualMachine {
    WireframeRect(FormPosition, Color),
    FilledRect(FormPosition, Color),
    Text(FormPosition, Color, String),
}
