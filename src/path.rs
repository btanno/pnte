use crate::*;
use windows::Win32::Graphics::{Direct2D::Common::*, Direct2D::*};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QuadraticBezierSegment {
    pub ctrl: Point<f32>,
    pub to: Point<f32>,
}

impl QuadraticBezierSegment {
    #[inline]
    pub fn new(ctrl: impl Into<Point<f32>>, to: impl Into<Point<f32>>) -> Self {
        Self {
            ctrl: ctrl.into(),
            to: to.into(),
        }
    }
}

impl From<QuadraticBezierSegment> for D2D1_QUADRATIC_BEZIER_SEGMENT {
    #[inline]
    fn from(value: QuadraticBezierSegment) -> Self {
        Self {
            point1: value.ctrl.into(),
            point2: value.to.into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CubicBezierSegment {
    pub c0: Point<f32>,
    pub c1: Point<f32>,
    pub to: Point<f32>,
}

impl CubicBezierSegment {
    #[inline]
    pub fn new(
        c0: impl Into<Point<f32>>,
        c1: impl Into<Point<f32>>,
        to: impl Into<Point<f32>>,
    ) -> Self {
        Self {
            c0: c0.into(),
            c1: c1.into(),
            to: to.into(),
        }
    }
}

impl From<CubicBezierSegment> for D2D1_BEZIER_SEGMENT {
    #[inline]
    fn from(value: CubicBezierSegment) -> Self {
        Self {
            point1: value.c0.into(),
            point2: value.c1.into(),
            point3: value.to.into(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum PathEnd {
    Open = D2D1_FIGURE_END_OPEN.0,
    Closed = D2D1_FIGURE_END_CLOSED.0,
}

impl From<PathEnd> for D2D1_FIGURE_END {
    #[inline]
    fn from(value: PathEnd) -> Self {
        Self(value as i32)
    }
}

pub struct PathBuilder {
    geometry: ID2D1PathGeometry,
    sink: ID2D1GeometrySink,
}

impl PathBuilder {
    #[inline]
    pub fn line_to(self, point: impl Into<Point<f32>>) -> Self {
        let point: Point<f32> = point.into();
        unsafe { self.sink.AddLine(point.into()) }
        self
    }

    #[inline]
    pub fn add_lines(self, points: &[Point<f32>]) -> Self {
        unsafe {
            let lines =
                std::slice::from_raw_parts(points.as_ptr() as *const D2D_POINT_2F, points.len());
            self.sink.AddLines(lines);
        }
        self
    }

    #[inline]
    pub fn quadratic_bezier_to(
        self,
        ctrl: impl Into<Point<f32>>,
        to: impl Into<Point<f32>>,
    ) -> Self {
        let segment = QuadraticBezierSegment::new(ctrl, to);
        unsafe {
            self.sink.AddQuadraticBezier(&segment.into());
        }
        self
    }

    #[inline]
    pub fn add_quadratic_beziers(self, segments: &[QuadraticBezierSegment]) -> Self {
        unsafe {
            let segments = std::slice::from_raw_parts(
                segments.as_ptr() as *const D2D1_QUADRATIC_BEZIER_SEGMENT,
                segments.len(),
            );
            self.sink.AddQuadraticBeziers(segments);
        }
        self
    }

    #[inline]
    pub fn cubic_bezier_to(
        self,
        c0: impl Into<Point<f32>>,
        c1: impl Into<Point<f32>>,
        to: impl Into<Point<f32>>,
    ) -> Self {
        let segment = CubicBezierSegment::new(c0, c1, to);
        unsafe {
            self.sink.AddBezier(&segment.into());
        }
        self
    }

    #[inline]
    pub fn add_cubic_beziers(self, segments: &[CubicBezierSegment]) -> Self {
        unsafe {
            let segments = std::slice::from_raw_parts(
                segments.as_ptr() as *const D2D1_BEZIER_SEGMENT,
                segments.len(),
            );
            self.sink.AddBeziers(segments);
        }
        self
    }

    #[inline]
    pub fn build(self, end: PathEnd) -> Result<Path> {
        unsafe {
            self.sink.EndFigure(end.into());
            self.sink.Close()?;
        }
        Ok(Path(self.geometry))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Path(ID2D1PathGeometry);

impl Path {
    #[inline]
    pub fn builder<T>(ctx: &Context<T>, start: impl Into<Point<f32>>) -> Result<PathBuilder>
    where
        T: Backend,
    {
        let geometry = unsafe { ctx.backend.d2d1_factory().CreatePathGeometry()? };
        let start: Point<f32> = start.into();
        let sink = unsafe { geometry.Open()? };
        unsafe {
            sink.BeginFigure(start.into(), D2D1_FIGURE_BEGIN_FILLED);
            sink.SetFillMode(D2D1_FILL_MODE_WINDING);
        }
        Ok(PathBuilder { geometry, sink })
    }
}

impl Fill for Path {
    #[inline]
    fn fill(&self, dc: &ID2D1DeviceContext5, brush: &ID2D1Brush) {
        unsafe {
            dc.FillGeometry(&self.0, brush, None);
        }
    }
}

impl Stroke for Path {
    #[inline]
    fn stroke(
        &self,
        dc: &ID2D1DeviceContext5,
        brush: &ID2D1Brush,
        width: f32,
        style: Option<&ID2D1StrokeStyle>,
    ) {
        unsafe {
            dc.DrawGeometry(&self.0, brush, width, style);
        }
    }
}
