use crate::*;
use std::path::Path;
use windows::core::{ComInterface, GUID, HSTRING};
use windows::Win32::{Foundation::GENERIC_READ, Graphics::Direct2D::*, Graphics::Imaging::*};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Interpolation {
    NearestNeighbor = D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR.0,
    Linear = D2D1_INTERPOLATION_MODE_LINEAR.0,
    Cubic = D2D1_INTERPOLATION_MODE_CUBIC.0,
    MultiSampleLinear = D2D1_INTERPOLATION_MODE_DEFINITION_MULTI_SAMPLE_LINEAR.0,
    Anisotropic = D2D1_INTERPOLATION_MODE_ANISOTROPIC.0,
    HighQualityCubic = D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC.0,
}

impl From<Interpolation> for D2D1_INTERPOLATION_MODE {
    fn from(value: Interpolation) -> Self {
        Self(value as i32)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Image(ID2D1Bitmap1);

impl Image {
    pub fn from_file<T>(ctx: &Context<T>, path: impl AsRef<Path>) -> Result<Self>
    where
        T: Backend,
    {
        let factory = &ctx.wic_imaging_factory;
        unsafe {
            let decoder = factory.CreateDecoderFromFilename(
                &HSTRING::from(path.as_ref().to_string_lossy().as_ref()),
                Some(&GUID::zeroed()),
                GENERIC_READ,
                WICDecodeMetadataCacheOnDemand,
            )?;
            let frame = decoder.GetFrame(0)?;
            let converter = factory.CreateFormatConverter()?;
            converter.Initialize(
                &frame,
                &GUID_WICPixelFormat32bppPBGRA,
                WICBitmapDitherTypeNone,
                None,
                1.0,
                WICBitmapPaletteTypeMedianCut,
            )?;
            let bitmap = ctx
                .d2d1_device_context
                .CreateBitmapFromWicBitmap(&converter, None)?
                .cast()?;
            Ok(Self(bitmap))
        }
    }

    #[inline]
    pub fn size(&self) -> Size<f32> {
        unsafe { self.0.GetSize().into() }
    }

    #[inline]
    pub fn pixel_size(&self) -> Size<u32> {
        unsafe { self.0.GetPixelSize().into() }
    }

    pub(crate) fn handle(&self) -> &ID2D1Bitmap1 {
        &self.0
    }
}

