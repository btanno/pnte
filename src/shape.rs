use crate::*;
use windows::Win32::Graphics::Direct2D::*;

impl Fill for Rect<f32> {
    #[inline]
    fn fill(&self, dc: &ID2D1DeviceContext5, brush: &ID2D1Brush) {
        unsafe {
            dc.FillRectangle(&(*self).into(), brush);
        }
    }
}

impl Stroke for Rect<f32> {
    #[inline]
    fn stroke(
        &self,
        dc: &ID2D1DeviceContext5,
        brush: &ID2D1Brush,
        width: f32,
        style: Option<&ID2D1StrokeStyle>,
    ) {
        unsafe {
            dc.DrawRectangle(&(*self).into(), brush, width, style);
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Line(pub Point<f32>, pub Point<f32>);

impl Line {
    #[inline]
    pub fn new(x0: impl Into<Point<f32>>, x1: impl Into<Point<f32>>) -> Self {
        Self(x0.into(), x1.into())
    }
}

impl Stroke for Line {
    #[inline]
    fn stroke(
        &self,
        dc: &ID2D1DeviceContext5,
        brush: &ID2D1Brush,
        width: f32,
        style: Option<&ID2D1StrokeStyle>,
    ) {
        unsafe {
            dc.DrawLine(self.0.into(), self.1.into(), brush, width, style);
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RoundedRect {
    pub rect: Rect<f32>,
    pub radius_x: f32,
    pub radius_y: f32,
}

impl RoundedRect {
    #[inline]
    pub fn new(rect: impl Into<Rect<f32>>, radius_x: f32, radius_y: f32) -> Self {
        Self {
            rect: rect.into(),
            radius_x,
            radius_y,
        }
    }
}

impl From<RoundedRect> for D2D1_ROUNDED_RECT {
    #[inline]
    fn from(value: RoundedRect) -> Self {
        Self {
            rect: value.rect.into(),
            radiusX: value.radius_x,
            radiusY: value.radius_y,
        }
    }
}

impl Fill for RoundedRect {
    #[inline]
    fn fill(&self, dc: &ID2D1DeviceContext5, brush: &ID2D1Brush) {
        unsafe {
            dc.FillRoundedRectangle(&(*self).into(), brush);
        }
    }
}

impl Stroke for RoundedRect {
    #[inline]
    fn stroke(
        &self,
        dc: &ID2D1DeviceContext5,
        brush: &ID2D1Brush,
        width: f32,
        style: Option<&ID2D1StrokeStyle>,
    ) {
        unsafe {
            dc.DrawRoundedRectangle(&(*self).into(), brush, width, style);
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle {
    pub center: Point<f32>,
    pub radius: f32,
}

impl Circle {
    #[inline]
    pub fn new(center: impl Into<Point<f32>>, radius: f32) -> Self {
        Self {
            center: center.into(),
            radius,
        }
    }

    #[inline]
    pub fn to_ellipse(self) -> Ellipse {
        Ellipse {
            center: self.center,
            radius_x: self.radius,
            radius_y: self.radius,
        }
    }
}

impl Fill for Circle {
    #[inline]
    fn fill(&self, dc: &ID2D1DeviceContext5, brush: &ID2D1Brush) {
        unsafe {
            dc.FillEllipse(&self.to_ellipse().into(), brush);
        }
    }
}

impl Stroke for Circle {
    #[inline]
    fn stroke(
        &self,
        dc: &ID2D1DeviceContext5,
        brush: &ID2D1Brush,
        width: f32,
        style: Option<&ID2D1StrokeStyle>,
    ) {
        unsafe {
            dc.DrawEllipse(&self.to_ellipse().into(), brush, width, style);
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ellipse {
    pub center: Point<f32>,
    pub radius_x: f32,
    pub radius_y: f32,
}

impl Ellipse {
    #[inline]
    pub fn new(center: impl Into<Point<f32>>, radius_x: f32, radius_y: f32) -> Self {
        Self {
            center: center.into(),
            radius_x,
            radius_y,
        }
    }
}

impl From<Ellipse> for D2D1_ELLIPSE {
    #[inline]
    fn from(value: Ellipse) -> Self {
        Self {
            point: value.center.into(),
            radiusX: value.radius_x,
            radiusY: value.radius_y,
        }
    }
}

impl Fill for Ellipse {
    #[inline]
    fn fill(&self, dc: &ID2D1DeviceContext5, brush: &ID2D1Brush) {
        unsafe {
            dc.FillEllipse(&(*self).into(), brush);
        }
    }
}

impl Stroke for Ellipse {
    #[inline]
    fn stroke(
        &self,
        dc: &ID2D1DeviceContext5,
        brush: &ID2D1Brush,
        width: f32,
        style: Option<&ID2D1StrokeStyle>,
    ) {
        unsafe {
            dc.DrawEllipse(&(*self).into(), brush, width, style);
        }
    }
}
