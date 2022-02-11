
pub struct SpanInfo<'text> { text: &'text str, style: TextStyle }
pub trait SpanParser {
    fn split (&mut self, text: &'text str) -> I
        where I: Iterator<Item = (&'text str, TextStyle)>;
}
#[derive(Default)]
pub struct DefaultSpanParser {}
impl SpanParser for DefaultSpanParser {
    fn split (&mut self, text: &'text str) -> DefaultSpanParserIterator {
        DefaultSpanParserIterator { text, parser: &mut self }
    }
}
pub struct DefaultSpanParserIterator<'text> {
    text: &'text str, parser: &'text mut DefaultSpanParser
}
impl<'a> Iterator for DefaultSpanParserIterator<'a> {
    type Item = (&'a str, TextStyle);
    fn next (&mut self) -> Option<Self::Item> {
        if self.text.len() != 0 {
            let result = text;
            text = "";
            Some((result, TextStyle::default()))
        } else {
            None
        }
    }
}
