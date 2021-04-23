use std::collections::HashMap;
#[derive(Debug, PartialEq, Copy, Clone)]
/// Error message type when failing to convert a hex code to RGB.
pub enum HtmlColorConversionError {
    InvalidStringLength,
    MissingHash,
    InvalidCharacter,
}

#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn from_rgb(r: u32, g: u32, b: u32) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    pub fn from_rgba(r: u32, g: u32, b: u32, a: u32) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    pub fn from_hex<S: AsRef<str>>(hash: S) -> Result<Self, HtmlColorConversionError> {
        let mut code = hash.as_ref().chars();

        if let Some(hash) = code.next() {
            if hash != '#' {
                return Err(HtmlColorConversionError::MissingHash);
            }
        } else {
            return Err(HtmlColorConversionError::InvalidStringLength);
        }

        let red1 = match code.next() {
            Some(red) => match red.to_digit(16) {
                Some(red) => red * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let red2 = match code.next() {
            Some(red) => match red.to_digit(16) {
                Some(red) => red,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let green1 = match code.next() {
            Some(green) => match green.to_digit(16) {
                Some(green) => green * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let green2 = match code.next() {
            Some(green) => match green.to_digit(16) {
                Some(green) => green,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let blue1 = match code.next() {
            Some(blue) => match blue.to_digit(16) {
                Some(blue) => blue * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let blue2 = match code.next() {
            Some(blue) => match blue.to_digit(16) {
                Some(blue) => blue,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };

        if code.next().is_some() {
            return Err(HtmlColorConversionError::InvalidStringLength);
        }

        let color = Self::from_rgb(red1 + red2, green1 + green2, blue1 + blue2);
        Ok(color)
    }

    pub fn from_palette(c: &str) -> Result<Self, String> {
        let mut palette = HashMap::new();
        palette.insert("black", "#000000");
        palette.insert("dark_blue", "#1D2B53");
        palette.insert("dark_purple", "#7E2553");
        palette.insert("dark_green", "#008751");
        palette.insert("brown", "#AB5236");
        palette.insert("dark_grey", "#5F574F");
        palette.insert("light_grey", "#C2C3C7");
        palette.insert("dark_grey", "#5F574F");
        palette.insert("white", "#FFF1E8");
        palette.insert("red", "#FF004D");
        palette.insert("orange", "#FFA300");
        palette.insert("yellow", "#FFEC27");
        palette.insert("green", "#00E436");
        palette.insert("blue", "#29ADFF");
        palette.insert("lavender", "#83769C");
        palette.insert("pink", "#FF77A8");
        palette.insert("light_peach", "#FFCCAA");

        let mut color: Option<Color> = None;

        for (k, _) in palette.iter() {
            if &c == k {
                color = Some(Self::from_hex(palette.get(&c).unwrap()).unwrap());
                break;
            } else {
                color = None
            }
        }

        match color {
            Some(color) => Ok(color),
            None => Err(String::from("Invalid color")),
        }
    }
}
