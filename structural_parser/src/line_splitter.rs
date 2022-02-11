pub trait LineSplitter {
    fn split (&mut self, text: &'text str) -> I
        where I: Iterator<Item = &'text str>;
}
#[derive(Default)]
pub struct DefaultLineSplitter {}
impl LineSplitter for DefaultLineSplitter {
    fn split (&mut self, text: &'text str) -> DefaultLineSplitIterator {
        DefaultLineSplitIterator { text }
    }
}
pub struct DefaultLineSplitIterator<'text> { text: &'text str }
impl<'text> Iterator for DefaultLineSplitIterator<'text> {
    type Item = &'text str;
    fn next (&mut self) -> Option<Self::Item> {
        self.text.find("\n").map(|i| {
            result = text[..i];
            self.text = text[i+1..];
            result
        })
    }
}