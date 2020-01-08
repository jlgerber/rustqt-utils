use hsluv::{hex_to_rgb, rgb_to_hex};

/// Enum repres
pub enum Color<'a> {
    Rgb { r: u8, g: u8, b: u8 },
    Rgba { r: u8, g: u8, b: u8, a: u8 },
    Hex(&'a str),
    Hsl(u8, f32, f32),
    Hsla(u8, f32, f32, f32),
    Red,
    Green,
    Blue,
    Grey,
    LightGrey,
    DarkGrey,
    AliceBlue,
    AntiqueWhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlanchedAlmond,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    ConrflowerBlue,
    Cornsilk,
    Crimson,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGlodenRod,
    DarkGray,
    DarkGreen,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkSlateGrey,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DimGrey,
    DodgerBlue,
    FireBrick,
    FloralWhite,
    ForestGreen,
    Fuchsia,
    Gainsboro,
    GhostWhite,
    Gold,
    GoldenRod,
    GreenYellow,
    HoneyDew,
    HotPink,
    IndiaRed,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    LavenderBlush,
    LawnGreen,
    LemonChiffon,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenRodYellow,
    LightGreen,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSlateGrey,
    LightSteelBlue,
    LightYellow,
    Lime,
    LimeGreen,
    Linen,
    Magenta,
    Maroon,
    MediumAquaMarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    Moccasin,
    NavajoWhite,
    Navy,
    OldLace,
    Olive,
    OliveDrab,
    Orange,
    OrangeRed,
    Orchid,
    PaleGoldenRod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaWhip,
    PeachPuff,
    Peru,
    Pink,
    Plum,
    PowderBlue,
    Purple,
    RebeccaPurple,
    Rosybrown,
    SaddleBrown,
    Salmon,
    SandyBrown,
    SeaGreen,
    SeaShell,
    Sienna,
    Silver,
    SkyBlue,
    SlateBlue,
    SlateGray,
    SlateGrey,
    Snow,
    SpringGreen,
    SteelBlue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    WhiteSmoke,
    Yellow,
    YellowGreen,
}

impl<'a> ToString for Color<'a> {
    fn to_string(&self) -> String {
        match self {
            Self::Rgb { r, g, b } => format!("rgb({}, {}, {})", r, g, b),
            Self::Rgba { r, g, b, a } => format!("rgba({}, {}, {}, {})", r, g, b, a),
            Self::Hex(ref val) => {
                // convert back and forth to keep it valid
                let color = hex_to_rgb(val);
                rgb_to_hex((color.0, color.1, color.2))
            }
            Self::Hsl(h, s, l) => {
                let sat = if s < &0.0 {
                    0.0
                } else if s > &100.0 {
                    100.0
                } else {
                    *s
                };
                let light = if l < &0.0 {
                    0.0
                } else if l > &100.0 {
                    100.0
                } else {
                    *l
                };
                format!("hsl({}, {}%, {}%)", h, sat, light)
            }
            Self::Hsla(h, s, l, a) => {
                let sat = if s < &0.0 {
                    0.0
                } else if s > &100.0 {
                    100.0
                } else {
                    *s
                };
                let light = if l < &0.0 {
                    0.0
                } else if l > &100.0 {
                    100.0
                } else {
                    *l
                };
                let alpha = if a < &0.0 {
                    0.0
                } else if a > &1.0 {
                    1.0
                } else {
                    *a
                };
                format!("hsla({}, {}%, {}%, {})", h, sat, light, alpha)
            }
            Self::Red => "red".to_string(),
            Self::Green => "green".to_string(),
            Self::Blue => "blue".to_string(),
            Self::Grey => "grey".to_string(),
            Self::LightGrey => "lightgrey".to_string(),
            Self::DarkGrey => "darkgrey".to_string(),
            Self::AliceBlue => "aliceblue".to_string(),
            Self::AntiqueWhite => "antiquewhite".to_string(),
            Self::Aqua => "aqua".to_string(),
            Self::Aquamarine => "aquamarine".to_string(),
            Self::Azure => "azure".to_string(),
            Self::Beige => "beige".to_string(),
            Self::Bisque => "bisque".to_string(),
            Self::Black => "black".to_string(),
            Self::BlanchedAlmond => "blanchedalmond".to_string(),
            Self::BlueViolet => "blueviolet".to_string(),
            Self::Brown => "brown".to_string(),
            Self::BurlyWood => "burlywood".to_string(),
            Self::CadetBlue => "cadetblue".to_string(),
            Self::Chartreuse => "chartreuse".to_string(),
            Self::Chocolate => "chocolate".to_string(),
            Self::Coral => "coral".to_string(),
            Self::ConrflowerBlue => "cornflowerblue".to_string(),
            Self::Cornsilk => "cornsilk".to_string(),
            Self::Crimson => "crimson".to_string(),
            Self::Cyan => "cyan".to_string(),
            Self::DarkBlue => "darkblue".to_string(),
            Self::DarkCyan => "darkcyan".to_string(),
            Self::DarkGlodenRod => "darkgoldenrod".to_string(),
            Self::DarkGray => "darkgray".to_string(),
            Self::DarkGreen => "darkgreen".to_string(),
            Self::DarkOrchid => "darkorchid".to_string(),
            Self::DarkRed => "darkred".to_string(),
            Self::DarkSalmon => "darksalmon".to_string(),
            Self::DarkSeaGreen => "darkseagreen".to_string(),
            Self::DarkSlateBlue => "darkslateblue".to_string(),
            Self::DarkSlateGray => "darkslategray".to_string(),
            Self::DarkSlateGrey => "darkslategrey".to_string(),
            Self::DarkTurquoise => "darkturquoise".to_string(),
            Self::DarkViolet => "darkviolet".to_string(),
            Self::DeepPink => "deeppink".to_string(),
            Self::DeepSkyBlue => "deepskyblue".to_string(),
            Self::DimGray => "dimgray".to_string(),
            Self::DimGrey => "dimgrey".to_string(),
            Self::DodgerBlue => "dodgerblue".to_string(),
            Self::FireBrick => "firebrick".to_string(),
            Self::FloralWhite => "floralwhite".to_string(),
            Self::ForestGreen => "forestgreen".to_string(),
            Self::Fuchsia => "fuchsia".to_string(),
            Self::Gainsboro => "gainsboro".to_string(),
            Self::GhostWhite => "ghostwhite".to_string(),
            Self::Gold => "gold".to_string(),
            Self::GoldenRod => "goldenrod".to_string(),
            Self::GreenYellow => "greenyellow".to_string(),
            Self::HoneyDew => "honeydew".to_string(),
            Self::HotPink => "hotpink".to_string(),
            Self::IndiaRed => "indiared".to_string(),
            Self::Indigo => "indigo".to_string(),
            Self::Ivory => "ivory".to_string(),
            Self::Khaki => "khaki".to_string(),
            Self::Lavender => "lavender".to_string(),
            Self::LavenderBlush => "lavenderblush".to_string(),
            Self::LawnGreen => "lawngreen".to_string(),
            Self::LemonChiffon => "lemonchiffon".to_string(),
            Self::LightBlue => "lightblue".to_string(),
            Self::LightCoral => "lightcoral".to_string(),
            Self::LightCyan => "lightcyan".to_string(),
            Self::LightGoldenRodYellow => "lightgoldenrodyellow".to_string(),
            Self::LightGreen => "lightgreen".to_string(),
            Self::LightPink => "lightpink".to_string(),
            Self::LightSalmon => "lightsalmon".to_string(),
            Self::LightSeaGreen => "lightseagreen".to_string(),
            Self::LightSkyBlue => "lightskyblue".to_string(),
            Self::LightSlateGray => "lightslategray".to_string(),
            Self::LightSlateGrey => "lightslategrey".to_string(),
            Self::LightSteelBlue => "lightsteelblue".to_string(),
            Self::LightYellow => "lightyellow".to_string(),
            Self::Lime => "lime".to_string(),
            Self::LimeGreen => "limegreen".to_string(),
            Self::Linen => "linen".to_string(),
            Self::Magenta => "magenta".to_string(),
            Self::Maroon => "maroon".to_string(),
            Self::MediumAquaMarine => "mediumaquamarine".to_string(),
            Self::MediumBlue => "mediumblue".to_string(),
            Self::MediumOrchid => "medumorchid".to_string(),
            Self::MediumPurple => "mediumpurple".to_string(),
            Self::MediumSeaGreen => "mediumseagreen".to_string(),
            Self::MediumSlateBlue => "mediumslateblue".to_string(),
            Self::MediumSpringGreen => "mediumspringgreen".to_string(),
            Self::MediumTurquoise => "mediumturquoise".to_string(),
            Self::MediumVioletRed => "mediumvioletred".to_string(),
            Self::MidnightBlue => "mednightblue".to_string(),
            Self::MintCream => "mintcream".to_string(),
            Self::MistyRose => "mistyrose".to_string(),
            Self::Moccasin => "moccasin".to_string(),
            Self::NavajoWhite => "navajowhite".to_string(),
            Self::Navy => "navy".to_string(),
            Self::OldLace => "oldlace".to_string(),
            Self::Olive => "olive".to_string(),
            Self::OliveDrab => "olivedrab".to_string(),
            Self::Orange => "orange".to_string(),
            Self::OrangeRed => "orangered".to_string(),
            Self::Orchid => "orchid".to_string(),
            Self::PaleGoldenRod => "palegoldenrod".to_string(),
            Self::PaleGreen => "palegreen".to_string(),
            Self::PaleTurquoise => "paleturquoise".to_string(),
            Self::PaleVioletRed => "palevioletred".to_string(),
            Self::PapayaWhip => "papayawhip".to_string(),
            Self::PeachPuff => "peachpuff".to_string(),
            Self::Peru => "peru".to_string(),
            Self::Pink => "pink".to_string(),
            Self::Plum => "plum".to_string(),
            Self::PowderBlue => "powderblue".to_string(),
            Self::Purple => "purple".to_string(),
            Self::RebeccaPurple => "rebeccapurple".to_string(),
            Self::Rosybrown => "rosybrown".to_string(),
            Self::SaddleBrown => "saddlebrown".to_string(),
            Self::Salmon => "salmon".to_string(),
            Self::SandyBrown => "sandybrown".to_string(),
            Self::SeaGreen => "seagreen".to_string(),
            Self::SeaShell => "seashell".to_string(),
            Self::Sienna => "sienna".to_string(),
            Self::Silver => "silver".to_string(),
            Self::SkyBlue => "skyblue".to_string(),
            Self::SlateBlue => "slateblue".to_string(),
            Self::SlateGray => "slategray".to_string(),
            Self::SlateGrey => "slategrey".to_string(),
            Self::Snow => "snow".to_string(),
            Self::SpringGreen => "springgreen".to_string(),
            Self::SteelBlue => "steelblue".to_string(),
            Self::Tan => "tan".to_string(),
            Self::Teal => "teal".to_string(),
            Self::Thistle => "thistle".to_string(),
            Self::Tomato => "tomato".to_string(),
            Self::Turquoise => "turquoise".to_string(),
            Self::Violet => "violet".to_string(),
            Self::Wheat => "wheat".to_string(),
            Self::White => "white".to_string(),
            Self::WhiteSmoke => "whitesmoke".to_string(),
            Self::Yellow => "yellow".to_string(),
            Self::YellowGreen => "yellowgreen".to_string(),
        }
    }
}

pub enum Size {
    Px(u8),
    Em(u8),
}

impl ToString for Size {
    fn to_string(&self) -> String {
        match self {
            Self::Px(val) => format!("{}px", val),
            Self::Em(val) => format!("{}em", val),
        }
    }
}

pub enum Border<'a> {
    Solid { size: Size, color: Color<'a> },
    None,
}

impl<'a> ToString for Border<'a> {
    fn to_string(&self) -> String {
        match self {
            Self::Solid { size, color } => {
                format!("{} solid {}", size.to_string(), color.to_string())
            }
            Self::None => "none".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_rgb_to_string() {
        let color = Color::Rgb {
            r: 255,
            g: 25,
            b: 200,
        }
        .to_string();
        assert_eq!(color.as_str(), "rgb(255, 25, 200)");
    }

    #[test]
    fn can_convert_rgba_to_string() {
        let color = Color::Rgba {
            r: 200,
            g: 201,
            b: 202,
            a: 250,
        }
        .to_string();
        assert_eq!(color.as_str(), "rgba(200, 201, 202, 250)");
    }

    #[test]
    fn can_convert_hex_string() {
        let color = Color::Hex("#FFFFFF").to_string();
        assert_eq!(color.as_str(), "#ffffff");
    }
    #[test]
    fn can_convert_hsl_string() {
        let color = Color::Hsl(200, 75.0, 50.0).to_string();
        assert_eq!(color.as_str(), "hsl(200, 75%, 50%)");
    }
    #[test]
    fn can_convert_hsla_string() {
        let color = Color::Hsla(200, 75.0, 50.0, 0.5).to_string();
        assert_eq!(color.as_str(), "hsla(200, 75%, 50%, 0.5)");
    }
    #[test]
    fn can_convert_red_to_string() {
        let color = Color::Red.to_string();
        assert_eq!(color.as_str(), "red");
    }

    #[test]
    fn can_convert_green_to_string() {
        let color = Color::Green.to_string();
        assert_eq!(color.as_str(), "green");
    }

    #[test]
    fn can_convert_blue_to_string() {
        let color = Color::Blue.to_string();
        assert_eq!(color.as_str(), "blue");
    }

    #[test]
    fn can_convert_grey_to_string() {
        let color = Color::Grey.to_string();
        assert_eq!(color.as_str(), "grey");
    }

    #[test]
    fn can_convert_lightgrey_to_string() {
        let color = Color::LightGrey.to_string();
        assert_eq!(color.as_str(), "lightgrey");
    }

    #[test]
    fn can_convert_darkgrey_to_string() {
        let color = Color::DarkGrey.to_string();
        assert_eq!(color.as_str(), "darkgrey");
    }
    //
    // sz tests
    //
    #[test]
    fn can_convert_px_to_string() {
        assert_eq!(Size::Px(80).to_string().as_str(), "80px");
    }

    #[test]
    fn can_convert_em_to_string() {
        assert_eq!(Size::Em(80).to_string().as_str(), "80em");
    }

    //
    // border tests
    //
    #[test]
    fn can_convert_solid_border_to_string() {
        assert_eq!(
            Border::Solid {
                size: Size::Px(1),
                color: Color::Red
            }
            .to_string()
            .as_str(),
            "1px solid red"
        );
    }

    #[test]
    fn can_convert_no_border_to_string() {
        assert_eq!(Border::None.to_string().as_str(), "none");
    }
}
