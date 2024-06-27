use crate::*;
use windows::core::Interface;
use windows::Win32::Graphics::{Direct2D::Common::*, Direct2D::*};

pub trait Brush {
    fn handle(&self) -> &ID2D1Brush;
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SolidColorBrush(ID2D1Brush);

impl SolidColorBrush {
    #[inline]
    pub fn new<T: Backend>(ctx: &Context<T>, color: impl Into<Rgba>) -> Result<Self> {
        let handle = unsafe {
            let color: Rgba = color.into();
            ctx.d2d1_device_context
                .CreateSolidColorBrush(&color.into(), None)?
        };
        Ok(Self(handle.cast().unwrap()))
    }
}

impl Brush for SolidColorBrush {
    #[inline]
    fn handle(&self) -> &ID2D1Brush {
        &self.0
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GradientStop {
    pub position: f32,
    pub color: Rgba,
}

impl GradientStop {
    #[inline]
    pub fn new(position: f32, color: impl Into<Rgba>) -> Self {
        Self {
            position,
            color: color.into(),
        }
    }
}

impl From<GradientStop> for D2D1_GRADIENT_STOP {
    #[inline]
    fn from(value: GradientStop) -> Self {
        Self {
            position: value.position,
            color: value.color.into(),
        }
    }
}

impl<C> From<(f32, C)> for GradientStop
where
    C: Into<Rgba>,
{
    #[inline]
    fn from(value: (f32, C)) -> Self {
        Self {
            position: value.0,
            color: value.1.into(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GradientMode {
    Clamp = D2D1_EXTEND_MODE_CLAMP.0,
    Mirror = D2D1_EXTEND_MODE_MIRROR.0,
    Wrap = D2D1_EXTEND_MODE_WRAP.0,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinearGradientBrush(ID2D1Brush);

impl LinearGradientBrush {
    #[inline]
    pub fn new<T, G>(
        ctx: &Context<T>,
        start: impl Into<Point<f32>>,
        end: impl Into<Point<f32>>,
        mode: GradientMode,
        stops: &[G],
    ) -> Result<Self>
    where
        T: Backend,
        G: Into<GradientStop> + Clone,
    {
        let dc = &ctx.d2d1_device_context;
        let stops: Vec<D2D1_GRADIENT_STOP> = stops
            .iter()
            .cloned()
            .map(|stop| D2D1_GRADIENT_STOP::from(stop.into()))
            .collect();
        let stops = unsafe {
            dc.CreateGradientStopCollection(
                &stops,
                D2D1_COLOR_SPACE_SRGB,
                D2D1_COLOR_SPACE_SRGB,
                D2D1_BUFFER_PRECISION_8BPC_UNORM,
                D2D1_EXTEND_MODE(mode as i32),
                D2D1_COLOR_INTERPOLATION_MODE_PREMULTIPLIED,
            )?
        };
        let brush = unsafe {
            dc.CreateLinearGradientBrush(
                &D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                    startPoint: D2D_POINT_2F::from(start.into()),
                    endPoint: D2D_POINT_2F::from(end.into()),
                },
                None,
                &stops,
            )?
        };
        Ok(Self(brush.cast().unwrap()))
    }
}

impl Brush for LinearGradientBrush {
    #[inline]
    fn handle(&self) -> &ID2D1Brush {
        &self.0
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RadialGradientBrush(ID2D1Brush);

impl RadialGradientBrush {
    #[inline]
    pub fn new<T, G>(
        ctx: &Context<T>,
        ellipse: impl Into<Ellipse>,
        offset: impl Into<Vector<f32>>,
        mode: GradientMode,
        stops: &[G],
    ) -> Result<Self>
    where
        T: Backend,
        G: Into<GradientStop> + Clone,
    {
        let dc = &ctx.d2d1_device_context;
        let stops: Vec<D2D1_GRADIENT_STOP> = stops
            .iter()
            .cloned()
            .map(|stop| D2D1_GRADIENT_STOP::from(stop.into()))
            .collect();
        let stops = unsafe {
            dc.CreateGradientStopCollection(
                &stops,
                D2D1_COLOR_SPACE_SRGB,
                D2D1_COLOR_SPACE_SRGB,
                D2D1_BUFFER_PRECISION_8BPC_UNORM,
                D2D1_EXTEND_MODE(mode as i32),
                D2D1_COLOR_INTERPOLATION_MODE_PREMULTIPLIED,
            )?
        };
        let ellipse: Ellipse = ellipse.into();
        let offset: Vector<f32> = offset.into();
        let brush = unsafe {
            dc.CreateRadialGradientBrush(
                &D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                    center: ellipse.center.into(),
                    radiusX: ellipse.radius_x,
                    radiusY: ellipse.radius_y,
                    gradientOriginOffset: offset.as_point().into(),
                },
                None,
                &stops,
            )?
        };
        Ok(Self(brush.cast().unwrap()))
    }
}

impl Brush for RadialGradientBrush {
    #[inline]
    fn handle(&self) -> &ID2D1Brush {
        &self.0
    }
}
