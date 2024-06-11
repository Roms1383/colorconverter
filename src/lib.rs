use red4ext_rs::{
    conv::NativeRepr,
    define_trait_plugin,
    plugin::{Plugin, Version},
    register_function,
};

struct ColorConverter;

impl Plugin for ColorConverter {
    const VERSION: Version = Version::new(1, 0, 0);

    fn register() {
        register_function!("ColorHexToRgba", color_hex_to_rgb);
    }
}

define_trait_plugin! (
    name: "ColorConverter",
    author: "Roms1383",
    plugin: ColorConverter
);

#[derive(Debug)]
#[repr(C)]
pub struct Color {
    alpha: u8,
    red: u8,
    blue: u8,
    green: u8,
}

unsafe impl NativeRepr for Color {
    const NAME: &'static str = "Color";
}

impl Default for Color {
    fn default() -> Self {
        Self {
            alpha: 0,
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

fn color_hex_to_rgb(hex: String) -> Color {
    if let Ok(color) = hex.as_str().parse::<csscolorparser::Color>() {
        let [red, green, blue, alpha] = color.to_rgba8();
        return Color {
            red,
            green,
            blue,
            alpha,
        };
    }
    red4ext_rs::error!("unable to convert {hex} to RGBA");
    Color::default()
}
