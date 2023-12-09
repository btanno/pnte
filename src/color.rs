use windows::Win32::Graphics::Direct2D::Common::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl From<D2D1_COLOR_F> for Rgba {
    #[inline]
    fn from(value: D2D1_COLOR_F) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

impl From<Rgba> for D2D1_COLOR_F {
    #[inline]
    fn from(value: Rgba) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

impl From<(f32, f32, f32, f32)> for Rgba {
    #[inline]
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
            a: value.3,
        }
    }
}
