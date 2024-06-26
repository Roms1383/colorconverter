use red4ext_rs::{
    conv::NativeRepr,
    define_trait_plugin,
    plugin::{Plugin, Version},
    register_function,
};

struct ColorConverter;

impl Plugin for ColorConverter {
    const VERSION: Version = Version::new(0, 1, 0);

    fn register() {
        register_function!("ColorHexToRgba", color_hex_to_rgba);
    }
}

define_trait_plugin! (
    name: "ColorConverter",
    author: "Roms1383",
    plugin: ColorConverter
);

/// See [RED4ext.SDK](https://github.com/WopsS/RED4ext.SDK/blob/master/include/RED4ext/Scripting/Natives/Generated/Color.hpp#L12)
#[derive(Debug, Default)]
#[repr(C)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

unsafe impl NativeRepr for Color {
    const NAME: &'static str = "Color";
}

fn color_hex_to_rgba(hex: String) -> Color {
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

#[cfg(test)]
mod memory {
    #[test]
    fn size() {
        static_assertions::const_assert_eq!(std::mem::size_of::<super::Color>(), 0x4);
    }
}

#[cfg(test)]
mod alpha {
    use crate::color_hex_to_rgba;

    #[test]
    fn with() {
        let color: super::Color = color_hex_to_rgba("AABBCCDD".into());
        assert_eq!(color.red, 170);
        assert_eq!(color.green, 187);
        assert_eq!(color.blue, 204);
        assert_eq!(color.alpha, 221);
    }

    #[test]
    fn without() {
        let color: super::Color = color_hex_to_rgba("AABBCC".into());
        assert_eq!(color.red, 170);
        assert_eq!(color.green, 187);
        assert_eq!(color.blue, 204);
        assert_eq!(color.alpha, 255);
    }
}
