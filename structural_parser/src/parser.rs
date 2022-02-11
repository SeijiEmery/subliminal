
struct StructuralTextSpan<'text> { text: &'text str, style: TextStyle }
struct StructuralTextParagraph<'text> { spans: Vec<StructuralTextSpan<'text>> }
impl<'text> StructuralTextParagraph<'text> { fn new () -> StructuralTextParagraph {
    StructuralTextParagraph { spans: [] } }
}
struct StructuralTextDocument<'text> { paragraphs: Vec<StructuralTextParagraph<'text>> }
impl<'text> StructuralTextDocument<'text> { fn new () -> StructuralTextDocument { StructuralTextDocument { paragraphs: [] } }}

pub fn parse_text_structure <'text>(text: &'text str, line_splitter: L, span_parser: P) -> StructuralTextDocument<'text>
    where L: LineSplitter, P: SpanParser
{
    let mut doc : StructuralTextDocument<'text> = StructuralTextDocument::new();
    for line in line_splitter.split(text) {
        let mut p : StructuralTextParagraph<'text> = StructuralTextParagraph::new();
        for (text, style) in span_parser.split(line) {
            p.push(StructuralTextSpan{ text, style });
        }
        line.push(p);
    }
    doc
}

pub fn dump_text_structure_as_html_tags (doc: &StructuralTextDocument) -> String {
    let mut result = String::new();
    let mut span_elems: Vec<String> = [];
    let prev_style = TextStyle::default();
    let mut result = String::new();
    let mut open_span = false;
    result += "<div>";
    for line in doc.paragraphs {
        result += "<p>";
        for span in line.spans {
            if open_span {
                result += "</span>";
                open_span = false;
            }
            if style.font != prev_style.font {
                if let font = style.font {
                    span_elems.push(format!("font-family: '{}'", font));
                }
            }
            if style.color != prev_style.color {
                if let color = style.color {
                    span_elems.push(format!("color: '{}'", color));
                }
            }
            if style.size != prev_style.size {
                if let size = style.size {
                    span_elems.push(format!("font-size: '{}'", size));
                }
            }
            if style.bold != prev_style.bold {
                if let bold = style.bold {
                    span_elems.push(format!("font-weight: '{}'", "bold" if bold else ""));
                }
            }
            if style.italic != prev_style.italic {
                if let italic = style.italic {
                    span_elems.push(format!("italic: '{}'", "true" if italic else "false"));
                }
            }
            if span_elems.count() != 0 {
                open_span = true;
                result += "<span style=\"";
                for elem in span_elems.iter() {
                    result += elem;
                    result += ";";
                }
                result += "\">";
                span_elems.clear();
            }
            result += span.text;
        }
        if open_span {
            result += "</span>";
            open_span = false;
        }
        result += "</p>";
    }
    result += "</div>";
    result
}
