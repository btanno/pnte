use crate::*;
use std::sync::Arc;
use windows::Win32::{Foundation::BOOL, Graphics::DirectWrite::*};
use windows::core::{HSTRING, Interface};

pub enum FontWeight {}

impl FontWeight {
    pub const THIN: f32 = DWRITE_FONT_WEIGHT_THIN.0 as f32;
    pub const ULTRA_LIGHT: f32 = DWRITE_FONT_WEIGHT_ULTRA_LIGHT.0 as f32;
    pub const LIGHT: f32 = DWRITE_FONT_WEIGHT_LIGHT.0 as f32;
    pub const SEMI_LIGHT: f32 = DWRITE_FONT_WEIGHT_SEMI_LIGHT.0 as f32;
    pub const REGULAR: f32 = DWRITE_FONT_WEIGHT_REGULAR.0 as f32;
    pub const MEDIUM: f32 = DWRITE_FONT_WEIGHT_MEDIUM.0 as f32;
    pub const SEMI_BOLD: f32 = DWRITE_FONT_WEIGHT_SEMI_BOLD.0 as f32;
    pub const ULTRA_BOLD: f32 = DWRITE_FONT_WEIGHT_ULTRA_BOLD.0 as f32;
    pub const HEAVY: f32 = DWRITE_FONT_WEIGHT_HEAVY.0 as f32;
    pub const ULTRA_BLACK: f32 = DWRITE_FONT_WEIGHT_ULTRA_BLACK.0 as f32;
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParagraphAlignment {
    Near = DWRITE_PARAGRAPH_ALIGNMENT_NEAR.0,
    Center = DWRITE_PARAGRAPH_ALIGNMENT_CENTER.0,
    Far = DWRITE_PARAGRAPH_ALIGNMENT_FAR.0,
}

impl From<ParagraphAlignment> for DWRITE_PARAGRAPH_ALIGNMENT {
    #[inline]
    fn from(value: ParagraphAlignment) -> Self {
        Self(value as i32)
    }
}

impl From<DWRITE_PARAGRAPH_ALIGNMENT> for ParagraphAlignment {
    #[inline]
    fn from(value: DWRITE_PARAGRAPH_ALIGNMENT) -> Self {
        match value {
            DWRITE_PARAGRAPH_ALIGNMENT_NEAR => Self::Near,
            DWRITE_PARAGRAPH_ALIGNMENT_CENTER => Self::Center,
            DWRITE_PARAGRAPH_ALIGNMENT_FAR => Self::Far,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontLineGapUsage {
    Default = DWRITE_FONT_LINE_GAP_USAGE_DEFAULT.0,
    Enabled = DWRITE_FONT_LINE_GAP_USAGE_ENABLED.0,
    Disabled = DWRITE_FONT_LINE_GAP_USAGE_DISABLED.0,
}

impl From<FontLineGapUsage> for DWRITE_FONT_LINE_GAP_USAGE {
    #[inline]
    fn from(value: FontLineGapUsage) -> Self {
        Self(value as i32)
    }
}

impl From<DWRITE_FONT_LINE_GAP_USAGE> for FontLineGapUsage {
    #[inline]
    fn from(value: DWRITE_FONT_LINE_GAP_USAGE) -> Self {
        match value {
            DWRITE_FONT_LINE_GAP_USAGE_DEFAULT => Self::Default,
            DWRITE_FONT_LINE_GAP_USAGE_ENABLED => Self::Enabled,
            DWRITE_FONT_LINE_GAP_USAGE_DISABLED => Self::Disabled,
            _ => unreachable!(),
        }
    }
}

impl Default for FontLineGapUsage {
    #[inline]
    fn default() -> Self {
        Self::Default
    }
}

pub mod line_spacing {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct Default {
        pub font_line_gap_usage: FontLineGapUsage,
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct Uniform {
        pub height: f32,
        pub baseline: f32,
        pub font_line_gap_usage: FontLineGapUsage,
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct Proportional {
        pub height: f32,
        pub baseline: f32,
        pub leading_before: f32,
        pub font_line_gap_usage: FontLineGapUsage,
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LineSpacing {
    Default(line_spacing::Default),
    Uniform(line_spacing::Uniform),
    Proportional(line_spacing::Proportional),
}

impl LineSpacing {
    #[inline]
    pub fn uniform(height: f32, baseline: f32, font_line_gap_usage: FontLineGapUsage) -> Self {
        Self::Uniform(line_spacing::Uniform {
            height,
            baseline,
            font_line_gap_usage,
        })
    }

    #[inline]
    pub fn proportional(
        height: f32,
        baseline: f32,
        leading_before: f32,
        font_line_gap_usage: FontLineGapUsage,
    ) -> Self {
        Self::Proportional(line_spacing::Proportional {
            height,
            baseline,
            leading_before,
            font_line_gap_usage,
        })
    }
}

impl Default for LineSpacing {
    #[inline]
    fn default() -> Self {
        Self::Default(line_spacing::Default {
            font_line_gap_usage: FontLineGapUsage::Default,
        })
    }
}

impl From<LineSpacing> for DWRITE_LINE_SPACING {
    #[inline]
    fn from(value: LineSpacing) -> Self {
        match value {
            LineSpacing::Default(v) => Self {
                method: DWRITE_LINE_SPACING_METHOD_DEFAULT,
                height: 0.0,
                baseline: 0.0,
                leadingBefore: 0.0,
                fontLineGapUsage: v.font_line_gap_usage.into(),
            },
            LineSpacing::Uniform(v) => Self {
                method: DWRITE_LINE_SPACING_METHOD_UNIFORM,
                height: v.height,
                baseline: v.baseline,
                leadingBefore: 0.0,
                fontLineGapUsage: v.font_line_gap_usage.into(),
            },
            LineSpacing::Proportional(v) => Self {
                method: DWRITE_LINE_SPACING_METHOD_PROPORTIONAL,
                height: v.height,
                baseline: v.baseline,
                leadingBefore: v.leading_before,
                fontLineGapUsage: v.font_line_gap_usage.into(),
            },
        }
    }
}

impl From<DWRITE_LINE_SPACING> for LineSpacing {
    #[inline]
    fn from(value: DWRITE_LINE_SPACING) -> Self {
        match value.method {
            DWRITE_LINE_SPACING_METHOD_DEFAULT => Self::Default(line_spacing::Default {
                font_line_gap_usage: value.fontLineGapUsage.into(),
            }),
            DWRITE_LINE_SPACING_METHOD_UNIFORM => Self::Uniform(line_spacing::Uniform {
                height: value.height,
                baseline: value.baseline,
                font_line_gap_usage: value.fontLineGapUsage.into(),
            }),
            DWRITE_LINE_SPACING_METHOD_PROPORTIONAL => {
                Self::Proportional(line_spacing::Proportional {
                    height: value.height,
                    baseline: value.baseline,
                    leading_before: value.leadingBefore,
                    font_line_gap_usage: value.fontLineGapUsage.into(),
                })
            }
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

impl Font<'_, '_> {
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
            let font_collection = factory
                .CreateFontCollectionFromFontSet(&font_set, DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC)?;
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HitTestResult {
    pub c: char,
    pub text_position: usize,
    pub inside: bool,
    pub trailing_hit: bool,
}

pub struct TextFormatBuilder<'a, F = (), S = ()> {
    factory: IDWriteFactory6,
    loader: Arc<FontFileLoader>,
    fnt: F,
    size: S,
    weight: f32,
    width: f32,
    slant: f32,
    italic: bool,
    locale: Option<&'a str>,
}

impl TextFormatBuilder<'_, (), ()> {
    fn new<T: Backend>(ctx: &Context<T>) -> Self {
        Self {
            factory: ctx.dwrite_factory.clone(),
            loader: ctx.font_file_loader.clone(),
            fnt: (),
            size: (),
            weight: FontWeight::REGULAR,
            width: 100.0,
            slant: 0.0,
            italic: false,
            locale: None,
        }
    }

    pub(crate) fn new_private(factory: &IDWriteFactory6, loader: &Arc<FontFileLoader>) -> Self {
        Self {
            factory: factory.clone(),
            loader: loader.clone(),
            fnt: (),
            size: (),
            weight: FontWeight::REGULAR,
            width: 100.0,
            slant: 0.0,
            italic: false,
            locale: None,
        }
    }
}

impl<'a, F, S> TextFormatBuilder<'a, F, S> {
    #[inline]
    pub fn font<'b, 'c>(self, font: Font<'b, 'c>) -> TextFormatBuilder<'a, Font<'b, 'c>, S> {
        TextFormatBuilder {
            factory: self.factory,
            loader: self.loader,
            fnt: font,
            size: self.size,
            weight: self.weight,
            width: self.width,
            slant: self.slant,
            italic: self.italic,
            locale: self.locale,
        }
    }

    #[inline]
    pub fn size(self, size: impl Into<f32>) -> TextFormatBuilder<'a, F, f32> {
        TextFormatBuilder {
            factory: self.factory,
            loader: self.loader,
            fnt: self.fnt,
            size: size.into(),
            weight: self.weight,
            width: self.width,
            slant: self.slant,
            italic: self.italic,
            locale: self.locale,
        }
    }

    #[inline]
    pub fn weight(mut self, weight: f32) -> Self {
        assert!((1.0..=1000.0).contains(&weight));
        self.weight = weight;
        self
    }

    #[inline]
    pub fn width(mut self, width: f32) -> Self {
        assert!(width > 0.0);
        self.width = width;
        self
    }

    #[inline]
    pub fn slant(mut self, slant: f32) -> Self {
        assert!((-90.0..=90.0).contains(&slant));
        self.slant = slant;
        self
    }

    #[inline]
    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    #[inline]
    pub fn locale(mut self, locale: &'a str) -> Self {
        self.locale = Some(locale);
        self
    }
}

impl TextFormatBuilder<'_, Font<'_, '_>, f32> {
    #[inline]
    pub fn build(self) -> Result<TextFormat> {
        let font_name = self.fnt.font_name();
        let font_collection = self.fnt.font_collection(&self.factory, &self.loader)?;
        let locale = HSTRING::from(self.locale.unwrap_or(""));
        let axis_values = [
            DWRITE_FONT_AXIS_VALUE {
                axisTag: DWRITE_FONT_AXIS_TAG_WEIGHT,
                value: self.weight,
            },
            DWRITE_FONT_AXIS_VALUE {
                axisTag: DWRITE_FONT_AXIS_TAG_WIDTH,
                value: self.width,
            },
            DWRITE_FONT_AXIS_VALUE {
                axisTag: DWRITE_FONT_AXIS_TAG_SLANT,
                value: self.slant,
            },
            DWRITE_FONT_AXIS_VALUE {
                axisTag: DWRITE_FONT_AXIS_TAG_ITALIC,
                value: if self.italic { 1.0f32 } else { 0.0 },
            },
        ];
        let format = unsafe {
            self.factory.CreateTextFormat(
                &HSTRING::from(font_name),
                font_collection.as_ref(),
                &axis_values,
                self.size,
                &locale,
            )?
        };
        Ok(TextFormat {
            format,
            font_name: font_name.into(),
            size: self.size,
            weight: self.weight,
            width: self.width,
            slant: self.slant,
            italic: self.italic,
        })
    }
}

#[derive(Clone, Debug)]
pub struct TextFormat {
    format: IDWriteTextFormat3,
    font_name: String,
    size: f32,
    weight: f32,
    width: f32,
    slant: f32,
    italic: bool,
}

impl TextFormat {
    #[inline]
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a, T: Backend>(ctx: &Context<T>) -> TextFormatBuilder<'a> {
        TextFormatBuilder::new(ctx)
    }

    #[inline]
    pub fn font_name(&self) -> &str {
        &self.font_name
    }

    #[inline]
    pub fn size(&self) -> f32 {
        self.size
    }

    #[inline]
    pub fn weight(&self) -> f32 {
        self.weight
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.width
    }

    #[inline]
    pub fn slant(&self) -> f32 {
        self.slant
    }

    #[inline]
    pub fn italic(&self) -> bool {
        self.italic
    }
}

impl PartialEq for TextFormat {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format
    }
}

impl Eq for TextFormat {}

pub trait Text {
    fn layout<T: Backend>(self, ctx: &Context<T>, format: &TextFormat) -> Result<TextLayout>;
}

pub struct TextLayoutBuilder<'a, T, Txt = (), Fmt = ()>
where
    T: Backend,
{
    ctx: &'a Context<T>,
    txt: Txt,
    format: Fmt,
    alignment: TextAlignment,
    paragraph_alignment: ParagraphAlignment,
    line_spacing: LineSpacing,
    size: Option<Size<f32>>,
}

impl<'a, T> TextLayoutBuilder<'a, T, (), ()>
where
    T: Backend,
{
    fn new(ctx: &'a Context<T>) -> Self {
        Self {
            ctx,
            txt: (),
            format: (),
            alignment: TextAlignment::Center,
            paragraph_alignment: ParagraphAlignment::Center,
            line_spacing: Default::default(),
            size: None,
        }
    }
}

impl<'a, T, Txt, Fmt> TextLayoutBuilder<'a, T, Txt, Fmt>
where
    T: Backend,
{
    #[inline]
    pub fn text<'b>(self, text: &'b str) -> TextLayoutBuilder<'a, T, &'b str, Fmt> {
        TextLayoutBuilder {
            ctx: self.ctx,
            txt: text,
            format: self.format,
            alignment: self.alignment,
            paragraph_alignment: self.paragraph_alignment,
            line_spacing: self.line_spacing,
            size: self.size,
        }
    }

    #[inline]
    pub fn format<'b>(
        self,
        format: &'b TextFormat,
    ) -> TextLayoutBuilder<'a, T, Txt, &'b TextFormat> {
        TextLayoutBuilder {
            ctx: self.ctx,
            txt: self.txt,
            format,
            alignment: self.alignment,
            paragraph_alignment: self.paragraph_alignment,
            line_spacing: self.line_spacing,
            size: self.size,
        }
    }

    #[inline]
    pub fn alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    #[inline]
    pub fn paragraph_alignment(mut self, paragraph_alignment: ParagraphAlignment) -> Self {
        self.paragraph_alignment = paragraph_alignment;
        self
    }

    #[inline]
    pub fn line_spacing(mut self, line_spacing: LineSpacing) -> Self {
        self.line_spacing = line_spacing;
        self
    }

    #[inline]
    pub fn size(mut self, size: impl Into<Size<f32>>) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl<T> TextLayoutBuilder<'_, T, &str, &TextFormat>
where
    T: Backend,
{
    pub fn build(self) -> Result<TextLayout> {
        let factory = &self.ctx.dwrite_factory;
        let s = HSTRING::from(self.txt);
        let layout: IDWriteTextLayout3 = unsafe {
            factory
                .CreateTextLayout(&s, &self.format.format, f32::MAX, f32::MAX)?
                .cast()?
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
                    length: s.len() as u32,
                },
            )?;
            typography
        };
        unsafe {
            layout.SetTextAlignment(self.alignment.into())?;
            layout.SetParagraphAlignment(self.paragraph_alignment.into())?;
            layout.SetLineSpacing(&self.line_spacing.into())?;
        }
        let size = unsafe {
            let size = self.size.unwrap_or_else(|| {
                let mut metrics = DWRITE_TEXT_METRICS1::default();
                layout.GetMetrics(&mut metrics).unwrap();
                (metrics.Base.width, metrics.Base.height).into()
            });
            layout.SetMaxWidth(size.width)?;
            layout.SetMaxHeight(size.height)?;
            size
        };
        Ok(TextLayout {
            layout,
            _typography: typography,
            format: self.format.clone(),
            chars: self.txt.chars().collect(),
            size,
        })
    }
}

#[derive(Clone, Debug)]
pub struct TextLayout {
    layout: IDWriteTextLayout3,
    format: TextFormat,
    _typography: IDWriteTypography,
    chars: Vec<char>,
    size: Size<f32>,
}

impl TextLayout {
    #[inline]
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>(ctx: &Context<T>) -> TextLayoutBuilder<T>
    where
        T: Backend,
    {
        TextLayoutBuilder::new(ctx)
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
    pub fn paragraph_alignment(&self) -> ParagraphAlignment {
        unsafe { self.layout.GetParagraphAlignment().into() }
    }

    #[inline]
    pub fn line_spacing(&self) -> LineSpacing {
        unsafe {
            let mut line_spacing = Default::default();
            self.layout.GetLineSpacing(&mut line_spacing).ok();
            line_spacing.into()
        }
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
        let text_position = metrics.textPosition as usize;
        Ok(HitTestResult {
            c: self.chars[text_position],
            text_position,
            inside: inside.as_bool(),
            trailing_hit: trailing_hit.as_bool(),
        })
    }

    pub(crate) fn handle(&self) -> &IDWriteTextLayout3 {
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

impl std::fmt::Display for TextLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chars().iter().collect::<String>())
    }
}

impl Text for &TextLayout {
    #[inline]
    fn layout<T: Backend>(self, _ctx: &Context<T>, _format: &TextFormat) -> Result<TextLayout> {
        Ok(self.clone())
    }
}

impl Text for &str {
    #[inline]
    fn layout<T: Backend>(self, ctx: &Context<T>, format: &TextFormat) -> Result<TextLayout> {
        TextLayout::new(ctx).text(self).format(format).build()
    }
}

impl Text for &String {
    #[inline]
    fn layout<T: Backend>(self, ctx: &Context<T>, format: &TextFormat) -> Result<TextLayout> {
        TextLayout::new(ctx).text(self).format(format).build()
    }
}
