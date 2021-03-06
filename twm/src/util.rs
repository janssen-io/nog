pub fn bytes_to_string(buffer: &[i8]) -> String {
    buffer
        .iter()
        .take_while(|b| **b != 0)
        .map(|byte| char::from(*byte as u8))
        .collect::<String>()
}

pub fn to_widestring(string: &str) -> Vec<u16> {
    string.encode_utf16().chain(std::iter::once(0)).collect()
}

pub type RGB = (i32, i32, i32);

pub fn rgb_to_hex(rgb: RGB) -> i32 {
    ((rgb.0 & 0xff) << 16) + ((rgb.1 & 0xff) << 8) + (rgb.2 & 0xff)
}

pub fn hex_to_rgb(hex: i32) -> RGB {
    ((hex >> 16) & 0xFF, (hex >> 8) & 0xFF, hex & 0xFF)
}

pub fn scale_color(color: i32, factor: f64) -> i32 {
    let (mut red, mut green, mut blue) = hex_to_rgb(color);

    red = (red as f64 * factor).round() as i32;
    green = (green as f64 * factor).round() as i32;
    blue = (blue as f64 * factor).round() as i32;

    rgb_to_hex((red, green, blue))
}
