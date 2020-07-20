/// A RGB color stored in linear space.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const BLACK: Color = Color::gray(0.0);
    pub const WHITE: Color = Color::gray(1.0);
    pub const RED: Color = Color::red(1.0);
    pub const GREEN: Color = Color::green(1.0);
    pub const BLUE: Color = Color::blue(1.0);
    pub const CYAN: Color = Color::cyan(1.0);
    pub const MAGENTA: Color = Color::magenta(1.0);
    pub const YELLOW: Color = Color::yellow(1.0);

    /// Creates a new RGBA color.
    ///
    /// Arguments are in linear space with `[0, 1]` range.
    #[inline]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    /// Creates a new RGB color with alpha 1.
    ///
    /// Arguments are in linear space with `[0, 1]` range.
    #[inline]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b, a: 1.0 }
    }

    /// Creates a new RGBA color.
    ///
    /// Arguments are in sRGB space with `[0, 255]` range.
    #[inline]
    pub fn srgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            r: u8_to_linear(r),
            g: u8_to_linear(g),
            b: u8_to_linear(b),
            a: u8_to_linear(a),
        }
    }

    /// Creates a new RGB color with alpha 1.
    ///
    /// Arguments are in sRGB space with `[0, 255]` range.
    #[inline]
    pub fn srgb8(r: u8, g: u8, b: u8) -> Self {
        Color {
            r: u8_to_linear(r),
            g: u8_to_linear(g),
            b: u8_to_linear(b),
            a: 1.0,
        }
    }

    /// Creates a new RGBA color.
    ///
    /// Components are in the `0xAARRGGBB` format commonly used in the web.
    #[inline]
    pub fn srgba32(rgb: u32) -> Self {
        Color {
            r: u8_to_linear((rgb >> 16) as u8),
            g: u8_to_linear((rgb >> 8) as u8),
            b: u8_to_linear(rgb as u8),
            a: u8_to_linear((rgb >> 24) as u8),
        }
    }

    /// Creates a new RGB color.
    ///
    /// Components are in the `0xRRGGBB` format commonly used in the web.
    #[inline]
    pub fn srgb32(rgb: u32) -> Self {
        Color {
            r: u8_to_linear((rgb >> 16) as u8),
            g: u8_to_linear((rgb >> 8) as u8),
            b: u8_to_linear(rgb as u8),
            a: 1.0,
        }
    }

    /// Creates a new color from HSL components.
    ///
    /// Argument `h` is in `[0, 360]` degrees, `s` and `l` in `[0, 1]` range.
    #[inline]
    pub fn hsl(h: f32, s: f32, l: f32) -> Self {
        hsl_to_rgb(h, s, l).into()
    }

    /// Converts this color into a 8-bit per component sRGBA array.
    ///
    /// Components are returned as a `[r, g, b, a]` array.
    #[inline]
    pub fn into_srgba8(self) -> [u8; 4] {
        [
            linear_to_u8(self.r),
            linear_to_u8(self.g),
            linear_to_u8(self.b),
            linear_to_u8(self.a),
        ]
    }

    /// Converts this color into a 8-bit per component sRGBA value.
    ///
    /// Components are returned in the `0xAARRGGBB` format commonly used in the web.
    #[inline]
    pub fn into_srgba32(self) -> u32 {
        let [r, g, b, a] = self.into_srgba8();
        b as u32 | (g as u32) << 8 | (r as u32) << 16 | (a as u32) << 24
    }

    #[inline]
    pub const fn red(r: f32) -> Self {
        Color::rgb(r, 0.0, 0.0)
    }

    #[inline]
    pub const fn green(g: f32) -> Self {
        Color::rgb(0.0, g, 0.0)
    }

    #[inline]
    pub const fn blue(b: f32) -> Self {
        Color::rgb(0.0, 0.0, b)
    }

    #[inline]
    pub const fn cyan(i: f32) -> Self {
        Color::rgb(0.0, i, i)
    }

    #[inline]
    pub const fn magenta(i: f32) -> Self {
        Color::rgb(i, 0.0, i)
    }

    #[inline]
    pub const fn yellow(i: f32) -> Self {
        Color::rgb(i, i, 0.0)
    }

    #[inline]
    pub const fn gray(i: f32) -> Self {
        Color::rgb(i, i, i)
    }

    #[inline]
    pub fn with_red(self, r: f32) -> Self {
        let Color { g, b, a, .. } = self;
        Color { r, g, b, a }
    }

    #[inline]
    pub fn with_green(self, g: f32) -> Self {
        let Color { r, b, a, .. } = self;
        Color { r, g, b, a }
    }

    #[inline]
    pub fn with_blue(self, b: f32) -> Self {
        let Color { r, g, a, .. } = self;
        Color { r, g, b, a }
    }

    #[inline]
    pub fn with_alpha(self, a: f32) -> Self {
        let Color { r, g, b, .. } = self;
        Color { r, g, b, a }
    }

    #[inline]
    pub fn opaque(self) -> Self {
        self.with_alpha(1.0)
    }

    #[inline]
    pub fn clamp(self) -> Self {
        self.map(|a| a.max(0.0).min(1.0))
    }

    #[inline]
    pub fn mix(self, other: Color, a: f32) -> Self {
        let a = a.max(0.0).min(1.0);
        self * (1.0 - a) + other * a
    }

    implement_map!(f32, r, g, b, a);
}

impl From<[f32; 4]> for Color {
    #[inline]
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        Color { r, g, b, a }
    }
}

impl From<[f32; 3]> for Color {
    #[inline]
    fn from([r, g, b]: [f32; 3]) -> Self {
        Color::rgb(r, g, b)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    #[inline]
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Color { r, g, b, a }
    }
}

impl From<(f32, f32, f32)> for Color {
    #[inline]
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Color::rgb(r, g, b)
    }
}

impl From<[u8; 4]> for Color {
    #[inline]
    fn from([r, g, b, a]: [u8; 4]) -> Self {
        Color::srgba8(r, g, b, a)
    }
}

impl From<[u8; 3]> for Color {
    #[inline]
    fn from([r, g, b]: [u8; 3]) -> Self {
        Color::srgb8(r, g, b)
    }
}

impl From<Color> for [f32; 4] {
    #[inline]
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl From<Color> for (f32, f32, f32, f32) {
    #[inline]
    fn from(c: Color) -> Self {
        (c.r, c.g, c.b, c.a)
    }
}

implement_ops!(Color, f32);

fn srgb_to_linear(s: f32) -> f32 {
    if s <= 0.04045 {
        s / 12.92
    } else {
        ((s + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(l: f32) -> f32 {
    if l <= 0.0031308 {
        l * 12.92
    } else {
        1.055 * l.powf(1.0 / 2.4) - 0.055
    }
}

fn u8_to_linear(srgb: u8) -> f32 {
    srgb_to_linear(srgb as f32 / 255.0)
}

fn linear_to_u8(linear: f32) -> u8 {
    (linear_to_srgb(linear.max(0.0).min(1.0)) * 255.0).round() as u8
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [f32; 3] {
    let a = s * l.min(1.0 - l);
    let f = move |n| {
        let k = (n + h / 30.0) % 12.0;
        l - a * f32::max(-1.0, f32::min(k - 3.0, 9.0 - k).min(1.0))
    };
    [f(0.0), f(8.0), f(4.0)]
}
