use crate::*;
use windows::Win32::Graphics::{Direct2D::Common::*, Direct2D::*};

pub trait Fill {
    fn fill(&self, dc: &ID2D1DeviceContext5, brush: &ID2D1Brush);
}

pub trait Stroke {
    fn stroke(
        &self,
        dc: &ID2D1DeviceContext5,
        brush: &ID2D1Brush,
        width: f32,
        style: Option<&ID2D1StrokeStyle>,
    );
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CapStyle {
    Flat = D2D1_CAP_STYLE_FLAT.0,
    Square = D2D1_CAP_STYLE_SQUARE.0,
    Round = D2D1_CAP_STYLE_ROUND.0,
    Triangle = D2D1_CAP_STYLE_TRIANGLE.0,
}

impl From<CapStyle> for D2D1_CAP_STYLE {
    #[inline]
    fn from(value: CapStyle) -> Self {
        D2D1_CAP_STYLE(value as i32)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LineJoin {
    Miter,
    Bevel,
    Round,
    MiterOrBevel(f32),
}

impl LineJoin {
    fn value(&self) -> (D2D1_LINE_JOIN, f32) {
        match self {
            Self::Miter => (D2D1_LINE_JOIN_MITER, 1.0),
            Self::Bevel => (D2D1_LINE_JOIN_BEVEL, 1.0),
            Self::Round => (D2D1_LINE_JOIN_ROUND, 1.0),
            Self::MiterOrBevel(miter_limit) => (D2D1_LINE_JOIN_MITER_OR_BEVEL, *miter_limit),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DashStyle<'a> {
    Solid,
    Dash,
    Dot,
    DashDot,
    DashDotDot,
    Custom(&'a [f32]),
}

impl<'a> DashStyle<'a> {
    fn value(&self) -> (D2D1_DASH_STYLE, Option<&[f32]>) {
        match self {
            Self::Solid => (D2D1_DASH_STYLE_SOLID, None),
            Self::Dash => (D2D1_DASH_STYLE_DASH, None),
            Self::Dot => (D2D1_DASH_STYLE_DOT, None),
            Self::DashDot => (D2D1_DASH_STYLE_DASH_DOT, None),
            Self::DashDotDot => (D2D1_DASH_STYLE_DASH_DOT_DOT, None),
            Self::Custom(dashes) => (D2D1_DASH_STYLE_CUSTOM, Some(dashes)),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dash<'a> {
    pub cap: CapStyle,
    pub style: DashStyle<'a>,
    pub offset: f32,
}

impl<'a> Default for Dash<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            cap: CapStyle::Flat,
            style: DashStyle::Solid,
            offset: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StrokeStyleProperties<'a> {
    pub start_cap: CapStyle,
    pub end_cap: CapStyle,
    pub line_join: LineJoin,
    pub dash: Option<Dash<'a>>,
}

impl<'a> Default for StrokeStyleProperties<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            start_cap: CapStyle::Flat,
            end_cap: CapStyle::Flat,
            line_join: LineJoin::Miter,
            dash: None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StrokeStyle(ID2D1StrokeStyle);

impl StrokeStyle {
    pub fn new<T>(ctx: &Context<T>, props: &StrokeStyleProperties) -> Result<Self>
    where
        T: Backend,
    {
        let (line_join, miter_limit) = props.line_join.value();
        let (dash_cap, dash_style, dash_offset, dashes) = match props.dash.as_ref() {
            Some(dash) => {
                let (style, dashes) = dash.style.value();
                (dash.cap.into(), style, dash.offset, dashes)
            }
            None => (D2D1_CAP_STYLE_FLAT, D2D1_DASH_STYLE_SOLID, 0.0, None),
        };
        let props = D2D1_STROKE_STYLE_PROPERTIES {
            startCap: props.start_cap.into(),
            endCap: props.end_cap.into(),
            dashCap: dash_cap,
            lineJoin: line_join,
            miterLimit: miter_limit,
            dashStyle: dash_style,
            dashOffset: dash_offset,
        };
        let handle = unsafe {
            ctx.backend
                .d2d1_factory()
                .CreateStrokeStyle(&props, dashes)?
        };
        Ok(Self(handle))
    }
}

pub struct DrawCommand {
    dc: ID2D1DeviceContext5,
}

impl DrawCommand {
    pub(crate) fn new(dc: &ID2D1DeviceContext5) -> Self {
        Self { dc: dc.clone() }
    }

    #[inline]
    pub fn clear(&self, color: impl Into<Rgba>) {
        unsafe {
            let color = D2D1_COLOR_F::from(color.into());
            self.dc.Clear(Some(&color));
        }
    }

    #[inline]
    pub fn fill(&self, object: &impl Fill, brush: &impl Brush) {
        object.fill(&self.dc, brush.handle());
    }

    #[inline]
    pub fn stroke(
        &self,
        object: &impl Stroke,
        brush: &impl Brush,
        width: f32,
        stroke_style: Option<&StrokeStyle>,
    ) {
        object.stroke(&self.dc, brush.handle(), width, stroke_style.map(|s| &s.0));
    }

    #[inline]
    pub fn draw_text(
        &self,
        layout: &TextLayout,
        position: impl Into<Point<f32>>,
        brush: &impl Brush,
    ) {
        unsafe {
            let position: Point<f32> = position.into();
            self.dc.DrawTextLayout(
                position.into(),
                layout.handle(),
                brush.handle(),
                D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT | D2D1_DRAW_TEXT_OPTIONS_CLIP,
            );
        }
    }

    #[inline]
    pub fn draw_image(
        &self,
        image: &Image,
        src_rect: Option<Rect<f32>>,
        dest_rect: impl Into<Rect<f32>>,
        opacity: Option<f32>,
        interpolation: Interpolation,
    ) {
        let src: Option<D2D_RECT_F> = src_rect.map(|src| src.into());
        let dest = D2D_RECT_F::from(dest_rect.into());
        unsafe {
            self.dc.DrawBitmap2(
                image.bitmap(),
                Some(&dest),
                opacity.unwrap_or(1.0),
                interpolation.into(),
                src.as_ref().map(|src| src as *const D2D_RECT_F),
                None,
            );
        }
    }

    #[inline]
    pub fn push_clip(&self, rect: impl Into<Rect<f32>>) {
        let rect: Rect<f32> = rect.into();
        unsafe {
            self.dc
                .PushAxisAlignedClip(&rect.into(), D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);
        }
    }

    #[inline]
    pub fn pop_clip(&self) {
        unsafe {
            self.dc.PopAxisAlignedClip();
        }
    }
}
