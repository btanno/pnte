use windows::Win32::Graphics::Direct2D::Common::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn as_vector(self) -> Vector<T> {
        Vector {
            x: self.x,
            y: self.y,
        }
    }
}

impl From<Point<f32>> for D2D_POINT_2F {
    #[inline]
    fn from(value: Point<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Point<u32>> for D2D_POINT_2U {
    #[inline]
    fn from(value: Point<u32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<D2D_POINT_2F> for Point<f32> {
    #[inline]
    fn from(value: D2D_POINT_2F) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<D2D_POINT_2U> for Point<u32> {
    #[inline]
    fn from(value: D2D_POINT_2U) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl<T> From<(T, T)> for Point<T> {
    #[inline]
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl From<Size<f32>> for D2D_SIZE_F {
    #[inline]
    fn from(value: Size<f32>) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

impl From<Size<u32>> for D2D_SIZE_U {
    #[inline]
    fn from(value: Size<u32>) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

impl From<D2D_SIZE_F> for Size<f32> {
    #[inline]
    fn from(value: D2D_SIZE_F) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

impl From<D2D_SIZE_U> for Size<u32> {
    #[inline]
    fn from(value: D2D_SIZE_U) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

impl<T> From<(T, T)> for Size<T> {
    #[inline]
    fn from(value: (T, T)) -> Self {
        Self {
            width: value.0,
            height: value.1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T,
}

impl<T> Rect<T> {
    pub const fn new(left: T, top: T, right: T, bottom: T) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    #[inline]
    pub fn from_points(lt: impl Into<Point<T>>, rb: impl Into<Point<T>>) -> Self {
        let lt: Point<T> = lt.into();
        let rb: Point<T> = rb.into();
        Self {
            left: lt.x,
            top: lt.y,
            right: rb.x,
            bottom: rb.y,
        }
    }

    #[inline]
    pub fn from_point_size(lt: impl Into<Point<T>>, size: impl Into<Size<T>>) -> Self
    where
        T: std::ops::Add<Output = T> + Clone,
    {
        let lt: Point<T> = lt.into();
        let size: Size<T> = size.into();
        Self {
            left: lt.x.clone(),
            top: lt.y.clone(),
            right: lt.x + size.width,
            bottom: lt.y + size.height,
        }
    }

    #[inline]
    pub fn left_top(self) -> Point<T> {
        Point::new(self.left, self.top)
    }

    #[inline]
    pub fn right_bottom(self) -> Point<T> {
        Point::new(self.right, self.bottom)
    }

    #[inline]
    pub fn size(self) -> Size<T>
    where
        T: std::ops::Sub<Output = T>,
    {
        Size::new(self.right - self.left, self.bottom - self.top)
    }
}

impl From<Rect<f32>> for D2D_RECT_F {
    #[inline]
    fn from(value: Rect<f32>) -> Self {
        Self {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        }
    }
}

impl From<Rect<u32>> for D2D_RECT_U {
    #[inline]
    fn from(value: Rect<u32>) -> Self {
        Self {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        }
    }
}

impl From<D2D_RECT_F> for Rect<f32> {
    #[inline]
    fn from(value: D2D_RECT_F) -> Self {
        Self {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        }
    }
}

impl From<D2D_RECT_U> for Rect<u32> {
    #[inline]
    fn from(value: D2D_RECT_U) -> Self {
        Self {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        }
    }
}

impl<T> From<(T, T, T, T)> for Rect<T> {
    #[inline]
    fn from(value: (T, T, T, T)) -> Self {
        Self {
            left: value.0,
            top: value.1,
            right: value.2,
            bottom: value.3,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn as_point(self) -> Point<T> {
        Point::new(self.x, self.y)
    }
}

impl From<Vector<f32>> for D2D_VECTOR_2F {
    #[inline]
    fn from(value: Vector<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<D2D_VECTOR_2F> for Vector<f32> {
    #[inline]
    fn from(value: D2D_VECTOR_2F) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl<T> From<(T, T)> for Vector<T> {
    #[inline]
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
