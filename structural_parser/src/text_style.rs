
pub struct TextStyle {
    font:  Option<String>,
    color: Option<Color>,
    size:  Option<f32>,
    italic: Option<bool>,
    bold:   Option<bool>,
}
impl Default for TextStyle {
    fn default () -> TextStyle {
        TextStyle { font: None, color: None, size: None, italic: false, bold: false }
    }
}
impl BitOr for TextStyle {
    type Output = Self;
    fn bitor (self, rhs: Self) -> Self {
        TextStyle {
            font:   font.or(rhs.font),
            color:  color.or(rhs.font),
            size:   size.or(rhs.size),
            italic: italic.or(rhs.italic),
            bold:   bold.or(rhs.bold),
        }
    }
}
