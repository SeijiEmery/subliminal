use std::hash::Hash;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Re-exported rusttype types.
pub mod rusttype {
    pub use full_rusttype::{
        point, Error, Font, Glyph, GlyphId, HMetrics, Point, PositionedGlyph, Rect, Scale,
        ScaledGlyph, SharedBytes, VMetrics,
    };
}

#[Derive(Debug, Default, Equals, Ord)]
pub struct Vec2<T = f32> { x: T, y: T }
//impl Default for Vec2<T> {
//    fn default () -> Vec2<T> { Vec2(0, 0) }
//}

#[Derive(Debug, Default, Equals, Ord)]
pub struct OptRect<T = f32> {
    pos: Option<Vec2<T>>,
    dim: Option<Vec2<T>>,
}
pub trait Color<T = f32> {
    fn rgb  (&self)         -> (T, T, T);
    fn rgba (&self)         -> (T, T, T, T);
    fn isTranparent (&self) -> bool;
}
pub struct TextStyle {
    font: Font,
    scale: Vec2,
    color: Color,
}
pub struct TextSpan<'text> {
    text:       &'text str,
    style:      Option<TextStyle>,
    bounds:     Option<Vec2>,
    localpos:   Option<Vec2>,
}
pub struct TextSection<'text> {
    text:       &'text str,/**/
    spans:      Vec<TextSpan<'text>>,
    style:      Option<TextStyle>,
    bounds:     Option<Vec2>,
    localpos:   Option<Vec2>,
}
#[derive(Debug, Default)]
pub struct TextView<'text> {
    text:           &'text str,
    text_sections:  Vec<TextSection<'text>>,
    text_bounds:    Option<Vec2>,
    text_pos:       Option<Vec2>,
    text_scale:     Option<Vec2>,
    dirty:          bool,

    /// pixel dimensions of this text view
    view_bounds:    Option<Vec2>,
    glyph_transform: Option<Mat4>,/**/
    view_transform:  Option<Mat4>,
}
impl<'text> TextView<'text> {
    fn new (text: &'text str) -> TextView<'text> {
        TextView { text, dirty: true, ..TextView::default() }
    }
    fn set_dirty (&mut self) { self.dirty = true }
    fn is_dirty (&self) -> bool { self.dirty }
    fn update (&mut self) {
        if (self.is_dirty()) {

        }
    }
}

#[derive(Debug, Hash)]
pub struct TextStyle {
    font:   Option<Font>,
    color:  Option<Color>,
    background_color: Option<Color>,
    bold:   bool,
    italic: bool,
}
impl Default for TextStyle {
    fn default () -> TextStyle {
        TextStyle { font: None, color: None, background_color: None, bold: false, italic: false }
    }
}

pub trait BasicTextModel {
    fn get_length              (&self) -> usize;
    fn get_text                (&self, offset: usize) -> &str;
}
pub trait TextModel : BasicTextModel {
    fn get_text_style          (&self, offset: usize) -> TextStyle;

    fn get_line_count          (&self) -> usize;
    fn get_paragraph           (&self, paragraph: usize) -> &str;
    fn get_paragraph_offset    (&self, paragraph: usize) -> usize;
    fn get_paragraph_hash      (&self, paragraph: usize) -> usize;

    fn get_paragraph_span_count (&self, paragraph: usize) -> usize;
    fn get_paragraph_span_text  (&self, paragraph: usize, span: usize) -> &str;
    fn get_paragraph_span_style (&self, paragraph: usize, span: usize) -> TextStyle;
    fn get_paragraph_span_hash  (&self, paragraph: usize, span: usize) -> usize;
}

pub trait LineSplitter : Default {
    fn begin               (&mut self);
    fn get_next_line_split (&mut self, text: &str) -> Option<(usize, &str)>;
}






#[derive(Hash)]
pub struct StringBackedTextModel<'text> {
    text: &'text str
}
impl<'text> BasicTextModel for StringBackedTextModel<'text> {
    fn get_length (&self) -> usize { self.text.len() }
    fn get_text   (&self, index: usize) -> &str { &self.text[index..] }
}
pub struct BasicOwnedTextModel { text: String }
impl BasicTextModel for BasicOwnedTextModel {
    fn get_length (&self) -> usize { self.text.len() }
    fn get_text (&self, index: usize) -> &str { &self.text[index..] }
}

pub fn borrowed_text_model<L = DefaultLineSplitter, P = DefaultSpanParser, M>
    (text: &str, line_splitter: L, span_parser: P) -> M
    where L: LineSplitter, P: TextSpanParser, M: TextModel
{
    make_text_model(StringBackedTextModel { text }, line_splitter, span_parser)
}
pub fn owned_text_model<L = DefaultLineSplitter, P = DefaultSpanParser, M>
    (text: &str, line_splitter: L, span_parser: P) -> M
    where L: LineSplitter, P: TextSpanParser, M: TextModel
{
    let text = String::from(text);
    make_text_model(BasicOwnedTextModel { text }, line_splitter, span_parser)
}
pub fn make_text_model<M, L, P>
    (text: M, line_splitter: L, span_parser: P) -> HashingTextModelImpl<M, L, P>
    where M: BasicTextModel, L: LineSplitter, P: TextSpanParser
{
    HashingTextModelImpl::new(text, line_splitter, span_parser)
}

pub struct HashingTextModelImpl<M, L, P>
    where M: BasicTextModel, L : LineSplitter, P: TextSpanParser
{
    text:          M,
    line_splitter: L,
    span_parser:   P,
    paragraphs:    Vec<TextParagraphModel>
}
impl HashingTextModelImpl<M, L, P> {
    fn new (text: M, line_splitter: L, span_parser: P) {
        let model = HashingTextModelImpl {
            text, line_splitter, span_parser, paragraphs: []
        };
        model.rebuild();
        model
    }
}
impl<M, L, P> HashingTextModelImpl<M, L, P>
    where M: TextModel, L: LineSplitter, P: TextSpanParser
{
    fn rebuild (&mut self) {
        self.paragraphs.clear();
        self.line_splitter.begin();
        let offset : usize = 0;
        let mut current_text = self.text.get_text(0);

        line_splitter.begin();
        let mut current_text = text;
        while let match = self.line_splitter.get_next_line_split()
    }
}


impl<M, L, P> BasicTextModel for HashingTextModelImpl<M, L, P> {
    fn get_length (&self) -> usize { text.get_length() }
    fn get_text (&self, offset: usize) { text.get_text(offset) }
    fn is_dirty (&self) -> bool {
        if line_count != text.get_length()
    }
}
impl TextModel for HashingTextModelImpl {
    fn get_text_style(&self, offset: usize) -> TextStyle {
        unimplemented!()
    }

    fn get_line_count(&self) -> usize {
        unimplemented!()
    }

    fn get_paragraph(&self, paragraph: usize) -> &str {
        unimplemented!()
    }

    fn get_paragraph_offset(&self, paragraph: usize) -> usize {
        unimplemented!()
    }

    fn get_paragraph_hash(&self, paragraph: usize) -> usize {
        unimplemented!()
    }

    fn get_paragraph_span_count(&self, paragraph: usize) -> usize {
        unimplemented!()
    }

    fn get_paragraph_span_text(&self, paragraph: usize, span: usize) -> &str {
        unimplemented!()
    }

    fn get_paragraph_span_style(&self, paragraph: usize, span: usize) -> _ {
        unimplemented!()
    }

    fn get_paragraph_span_hash(&self, paragraph: usize, span: usize) -> usize {
        unimplemented!()
    }
}


