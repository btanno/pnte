use crate::*;
use std::cell::UnsafeCell;
use windows::core::{ComInterface, HSTRING};
use windows::Win32::{Foundation::BOOL, Graphics::DirectWrite::*};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontWeight {
    Thin = DWRITE_FONT_WEIGHT_THIN.0,
    UltraLight = DWRITE_FONT_WEIGHT_ULTRA_LIGHT.0,
    Light = DWRITE_FONT_WEIGHT_LIGHT.0,
    SemiLight = DWRITE_FONT_WEIGHT_SEMI_LIGHT.0,
    Regular = DWRITE_FONT_WEIGHT_REGULAR.0,
    Medium = DWRITE_FONT_WEIGHT_MEDIUM.0,
    SemiBold = DWRITE_FONT_WEIGHT_SEMI_BOLD.0,
    Bold = DWRITE_FONT_WEIGHT_BOLD.0,
    UltraBold = DWRITE_FONT_WEIGHT_ULTRA_BOLD.0,
    Heavy = DWRITE_FONT_WEIGHT_HEAVY.0,
    UltraBlack = DWRITE_FONT_WEIGHT_ULTRA_BLACK.0,
}

impl Default for FontWeight {
    #[inline]
    fn default() -> Self {
        Self::Regular
    }
}

impl From<FontWeight> for DWRITE_FONT_WEIGHT {
    #[inline]
    fn from(value: FontWeight) -> Self {
        Self(value as i32)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStyle {
    Normal = DWRITE_FONT_STYLE_NORMAL.0,
    Oblique = DWRITE_FONT_STYLE_OBLIQUE.0,
    Italic = DWRITE_FONT_STYLE_ITALIC.0,
}

impl Default for FontStyle {
    #[inline]
    fn default() -> Self {
        Self::Normal
    }
}

impl From<FontStyle> for DWRITE_FONT_STYLE {
    #[inline]
    fn from(value: FontStyle) -> Self {
        Self(value as i32)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStretch {
    Undefined = DWRITE_FONT_STRETCH_UNDEFINED.0,
    UltraCondensed = DWRITE_FONT_STRETCH_ULTRA_CONDENSED.0,
    ExtraCondensed = DWRITE_FONT_STRETCH_EXTRA_CONDENSED.0,
    Condensed = DWRITE_FONT_STRETCH_CONDENSED.0,
    SemiCondensed = DWRITE_FONT_STRETCH_SEMI_CONDENSED.0,
    Medium = DWRITE_FONT_STRETCH_MEDIUM.0,
    SemiExpanded = DWRITE_FONT_STRETCH_SEMI_EXPANDED.0,
    Expanded = DWRITE_FONT_STRETCH_EXPANDED.0,
    ExtraExpanded = DWRITE_FONT_STRETCH_EXTRA_EXPANDED.0,
    UltraExpanded = DWRITE_FONT_STRETCH_ULTRA_EXPANDED.0,
}

impl Default for FontStretch {
    #[inline]
    fn default() -> Self {
        Self::Medium
    }
}

impl From<FontStretch> for DWRITE_FONT_STRETCH {
    #[inline]
    fn from(value: FontStretch) -> Self {
        Self(value as i32)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextAlignment {
    Leading = DWRITE_TEXT_ALIGNMENT_LEADING.0,
    Center = DWRITE_TEXT_ALIGNMENT_CENTER.0,
    Trailing = DWRITE_TEXT_ALIGNMENT_TRAILING.0,
    Justified = DWRITE_TEXT_ALIGNMENT_JUSTIFIED.0,
}

impl From<TextAlignment> for DWRITE_TEXT_ALIGNMENT {
    #[inline]
    fn from(value: TextAlignment) -> Self {
        Self(value as i32)
    }
}

impl From<DWRITE_TEXT_ALIGNMENT> for TextAlignment {
    #[inline]
    fn from(value: DWRITE_TEXT_ALIGNMENT) -> Self {
        match value {
            DWRITE_TEXT_ALIGNMENT_LEADING => Self::Leading,
            DWRITE_TEXT_ALIGNMENT_CENTER => Self::Center,
            DWRITE_TEXT_ALIGNMENT_TRAILING => Self::Trailing,
            DWRITE_TEXT_ALIGNMENT_JUSTIFIED => Self::Justified,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Font<'a, 'b> {
    System(&'b str),
    File(&'a std::path::Path, &'b str),
    Memory(&'a [u8], &'b str),
}

impl<'a, 'b> Font<'a, 'b> {
    fn font_name(&self) -> &str {
        match self {
            Self::System(name) => name,
            Self::File(_, name) => name,
            Self::Memory(_, name) => name,
        }
    }

    fn font_collection(
        &self,
        factory: &IDWriteFactory6,
        loader: &FontFileLoader,
    ) -> Result<Option<IDWriteFontCollection>> {
        if let Self::System(_) = self {
            return Ok(None);
        }
        unsafe {
            let set_builder: IDWriteFontSetBuilder1 = factory.CreateFontSetBuilder()?.cast()?;
            let font_file = match self {
                Self::File(path, _) => factory.CreateFontFileReference(
                    &HSTRING::from(path.to_string_lossy().as_ref()),
                    None,
                )?,
                Self::Memory(data, _) => loader.handle().CreateInMemoryFontFileReference(
                    factory,
                    data.as_ptr() as *const std::ffi::c_void,
                    data.len() as u32,
                    None,
                )?,
                _ => unreachable!(),
            };
            set_builder.AddFontFile(&font_file)?;
            let font_set = set_builder.CreateFontSet()?;
            let font_collection = factory.CreateFontCollectionFromFontSet(&font_set)?;
            Ok(Some(font_collection.cast().unwrap()))
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct FontPoint(pub f32);

impl From<FontPoint> for f32 {
    #[inline]
    fn from(value: FontPoint) -> Self {
        value.0 * 96.0 / 72.0
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct TextStyle {
    pub weight: FontWeight,
    pub font_style: FontStyle,
    pub stretch: FontStretch,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HitTestResult {
    pub text_position: usize,
    pub inside: bool,
    pub trailing_hit: bool,
}

#[derive(Debug)]
pub struct TextFormat {
    format: IDWriteTextFormat,
    _not_sync: UnsafeCell<()>, // !Sync
}

impl TextFormat {
    pub fn new<T>(
        ctx: &Context<T>,
        font: Font,
        size: impl Into<f32>,
        style: Option<TextStyle>,
        locale: Option<&str>,
    ) -> Result<Self>
    where
        T: Backend,
    {
        let factory = &ctx.dwrite_factory;
        let loader = &ctx.font_file_loader;
        let font_name = font.font_name();
        let font_collection = font.font_collection(factory, loader)?;
        let style = style.unwrap_or_default();
        let locale = HSTRING::from(locale.unwrap_or(""));
        let format = unsafe {
            factory.CreateTextFormat(
                &HSTRING::from(font_name),
                font_collection.as_ref(),
                style.weight.into(),
                style.font_style.into(),
                style.stretch.into(),
                size.into(),
                &locale,
            )?
        };
        Ok(Self {
            format,
            _not_sync: UnsafeCell::new(()),
        })
    }

    pub(crate) fn from_handle(format: &IDWriteTextFormat) -> Self {
        Self {
            format: format.clone(),
            _not_sync: UnsafeCell::new(()),
        }
    }

    pub(crate) fn handle(&self) -> &IDWriteTextFormat {
        &self.format
    }
}

impl PartialEq for TextFormat {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format
    }
}

impl Eq for TextFormat {}

impl Clone for TextFormat {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            format: self.format.clone(),
            _not_sync: UnsafeCell::new(()),
        }
    }
}

pub trait Text {
    fn layout<T: Backend>(self, ctx: &Context<T>, format: &TextFormat) -> Result<TextLayout>;
}

#[derive(Debug)]
pub struct TextLayout {
    layout: IDWriteTextLayout,
    format: TextFormat,
    typography: IDWriteTypography,
    chars: Vec<char>,
    size: Size<f32>,
    _not_sync: UnsafeCell<()>,
}

impl TextLayout {
    pub fn new<T>(
        ctx: &Context<T>,
        text: impl AsRef<str>,
        format: &TextFormat,
        alignment: TextAlignment,
        size: Option<Size<f32>>,
    ) -> Result<Self>
    where
        T: Backend,
    {
        let factory = &ctx.dwrite_factory;
        let text = text.as_ref();
        let s = HSTRING::from(text);
        let layout = unsafe {
            factory.CreateTextLayout(s.as_wide(), &format.format, std::f32::MAX, std::f32::MAX)?
        };
        let typography = unsafe {
            let typography = factory.CreateTypography()?;
            typography.AddFontFeature(DWRITE_FONT_FEATURE {
                nameTag: DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES,
                parameter: 0,
            })?;
            layout.SetTypography(
                &typography,
                DWRITE_TEXT_RANGE {
                    startPosition: 0,
                    length: s.as_wide().len() as u32,
                },
            )?;
            typography
        };
        unsafe {
            layout.SetTextAlignment(alignment.into())?;
            layout.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?;
        }
        let size = unsafe {
            let size = size.unwrap_or_else(|| {
                let mut metrics = DWRITE_TEXT_METRICS::default();
                layout.GetMetrics(&mut metrics).unwrap();
                (metrics.width, metrics.height).into()
            });
            layout.SetMaxWidth(size.width)?;
            layout.SetMaxHeight(size.height)?;
            size
        };
        Ok(Self {
            layout,
            typography,
            format: format.clone(),
            chars: text.chars().collect(),
            size,
            _not_sync: UnsafeCell::new(()),
        })
    }

    #[inline]
    pub fn format(&self) -> &TextFormat {
        &self.format
    }

    #[inline]
    pub fn size(&self) -> Size<f32> {
        self.size
    }

    #[inline]
    pub fn alignment(&self) -> TextAlignment {
        unsafe { self.layout.GetTextAlignment().into() }
    }

    #[inline]
    pub fn chars(&self) -> &[char] {
        &self.chars
    }

    #[inline]
    pub fn hit_test(&self, pt: impl Into<Point<f32>>) -> Result<HitTestResult> {
        let pt: Point<f32> = pt.into();
        let mut trailing_hit = BOOL::default();
        let mut inside = BOOL::default();
        let mut metrics = DWRITE_HIT_TEST_METRICS::default();
        unsafe {
            self.layout
                .HitTestPoint(pt.x, pt.y, &mut trailing_hit, &mut inside, &mut metrics)?;
        }
        Ok(HitTestResult {
            text_position: metrics.textPosition as usize,
            inside: inside.as_bool(),
            trailing_hit: trailing_hit.as_bool(),
        })
    }

    pub(crate) fn handle(&self) -> &IDWriteTextLayout {
        &self.layout
    }
}

impl PartialEq for TextLayout {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.layout == other.layout
    }
}

impl Eq for TextLayout {}

impl Clone for TextLayout {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            layout: self.layout.clone(),
            format: self.format.clone(),
            typography: self.typography.clone(),
            chars: self.chars.clone(),
            size: self.size,
            _not_sync: UnsafeCell::new(()),
        }
    }
}

impl ToString for TextLayout {
    #[inline]
    fn to_string(&self) -> String {
        self.chars.iter().collect()
    }
}

impl<'a> Text for &'a TextLayout {
    fn layout<T: Backend>(self, _ctx: &Context<T>, _format: &TextFormat) -> Result<TextLayout> {
        Ok(self.clone())
    }
}

impl<'a> Text for &'a str {
    fn layout<T: Backend>(self, ctx: &Context<T>, format: &TextFormat) -> Result<TextLayout> {
        let layout = TextLayout::new(ctx, self, format, TextAlignment::Center, None)?;
        Ok(layout)
    }
}

impl<'a> Text for &'a String {
    fn layout<T: Backend>(self, ctx: &Context<T>, format: &TextFormat) -> Result<TextLayout> {
        let layout = TextLayout::new(ctx, self, format, TextAlignment::Center, None)?;
        Ok(layout)
    }
}
