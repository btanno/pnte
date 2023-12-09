pub mod d2d1;
pub mod d3d11;
pub mod d3d12;

use crate::*;
use std::sync::Arc;
use windows::Win32::{
    Graphics::Direct2D::*,
    Graphics::DirectWrite::*,
    Graphics::Imaging::D2D::*,
    Graphics::Imaging::*,
    System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
};

pub trait Target {
    fn bitmap(&self) -> &ID2D1Bitmap1;
    fn size(&self) -> Size<f32>;
    fn pixel_size(&self) -> Size<u32>;
}

pub trait Backend {
    type RenderTarget: Target;

    fn d2d1_factory(&self) -> &ID2D1Factory6;
    fn d2d1_device(&self) -> &ID2D1Device5;

    fn begin_draw(&self, _target: &Self::RenderTarget) {}
    fn end_draw(&self, _target: &Self::RenderTarget) -> Result<()> {
        Ok(())
    }
}

pub(crate) struct FontFileLoader {
    factory: IDWriteFactory6,
    loader: IDWriteInMemoryFontFileLoader,
}

impl FontFileLoader {
    fn new(factory: &IDWriteFactory6) -> Result<Self> {
        let loader = unsafe { factory.CreateInMemoryFontFileLoader()? };
        unsafe {
            factory.RegisterFontFileLoader(&loader)?;
        }
        Ok(Self {
            factory: factory.clone(),
            loader,
        })
    }

    pub(crate) fn handle(&self) -> &IDWriteInMemoryFontFileLoader {
        &self.loader
    }
}

impl Drop for FontFileLoader {
    fn drop(&mut self) {
        unsafe {
            self.factory.UnregisterFontFileLoader(&self.loader).ok();
        }
    }
}

#[derive(Clone)]
pub struct Context<T: Backend> {
    pub(crate) backend: Arc<T>,
    pub(crate) d2d1_device_context: ID2D1DeviceContext5,
    pub(crate) dwrite_factory: IDWriteFactory6,
    pub(crate) font_file_loader: Arc<FontFileLoader>,
    pub(crate) wic_imaging_factory: IWICImagingFactory2,
    pub(crate) default_text_format: TextFormat,
}

impl<T> Context<T>
where
    T: Backend,
{
    #[inline]
    pub fn new(backend: T) -> Result<Self> {
        let d2d1_device_context = unsafe {
            backend.d2d1_device().CreateDeviceContext6(
                D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS,
            )?
        };
        let dwrite_factory: IDWriteFactory6 =
            unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)? };
        let wic_imaging_factory =
            unsafe { CoCreateInstance(&CLSID_WICImagingFactory2, None, CLSCTX_INPROC_SERVER)? };
        let font_file_loader = FontFileLoader::new(&dwrite_factory)?;
        let default_text_format = TextFormat::new_impl(
            &dwrite_factory,
            &font_file_loader,
            Font::System(""),
            FontPoint(14.0),
            None,
            None,
        )?;
        Ok(Self {
            backend: Arc::new(backend),
            d2d1_device_context,
            dwrite_factory,
            font_file_loader: Arc::new(font_file_loader),
            wic_imaging_factory,
            default_text_format,
        })
    }

    #[inline]
    pub fn set_dpi(&mut self, dpi_x: f32, dpi_y: f32) {
        unsafe {
            self.d2d1_device_context.SetDpi(dpi_x, dpi_y);
        }
    }

    #[inline]
    pub fn set_scale_factor(&mut self, scale: f32) {
        let scale = scale * 96.0;
        self.set_dpi(scale, scale);
    }

    #[inline]
    pub fn set_default_text_format(&mut self, format: &TextFormat) {
        self.default_text_format = format.clone();
    }

    #[inline]
    pub fn draw<R>(
        &self,
        target: &T::RenderTarget,
        f: impl FnOnce(DrawCommand<T>) -> R,
    ) -> Result<R> {
        let ctx = &self.d2d1_device_context;
        self.backend.begin_draw(target);
        unsafe {
            ctx.SetTarget(target.bitmap());
            ctx.BeginDraw();
        }
        let ret = f(DrawCommand::new(self));
        unsafe {
            ctx.EndDraw(None, None)?;
            ctx.SetTarget(None);
        }
        self.backend.end_draw(target)?;
        Ok(ret)
    }
}
