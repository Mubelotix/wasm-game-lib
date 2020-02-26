use super::font::Font;
use crate::graphics::drawable::Drawable;
use crate::graphics::canvas::Canvas;

const EMPTY_STR: &str = "";
const PX_STR: &str = "px";

/// A struct representing the style of a [Text](struct.Text.html).
pub struct TextStyle {
    /// Italic
    pub italic: bool,
    /// Bold
    pub bold: bool,
    /// Underlined is not supported for now!
    pub underlined: bool,
}

impl Default for TextStyle {
    fn default() -> TextStyle {
        TextStyle {
            italic: false,
            bold: false,
            underlined: false
        }
    }
}

/// A text drawable on a [Canvas](../canvas/struct.Canvas.html).
pub struct Text<'a> {
    /// The coords of the text in px.
    pub coords: (usize, usize),
    /// The [font](../font/struct.Font.html) of the text.
    pub font: &'a Font,
    /// The text.
    pub text: &'a str,
    /// The [style](struct.TextStyle.html) of the text (bold/italic...)
    pub style: TextStyle,
    /// The character_size. example: (14, "px")
    pub character_size: (usize, &'a str)
}

impl<'a> Text<'a> {
    /// Create a new text with default values.
    pub fn new(font: &'a Font) -> Text<'a> {
        Text {
            coords: (0,0),
            font,
            text: EMPTY_STR,
            style: TextStyle::default(),
            character_size: (26, PX_STR)
        }
    }

    /// Create a new text with some default values.
    pub fn new_with_text_and_coords(font: &'a Font, text: &'a str, coords: (usize, usize)) -> Text<'a> {
        Text {
            coords,
            font,
            text,
            style: TextStyle::default(),
            character_size: (26, PX_STR)
        }
    }

    /// Create a new text with no default value.
    pub fn new_with_options(font: &'a Font, text: &'a str, coords: (usize, usize), style: TextStyle, character_size: (usize, &'a str)) -> Text<'a> {
        Text {
            coords,
            font,
            text,
            style,
            character_size
        }
    }

    /// Set the displayed text.
    pub fn set_text(&mut self, text: &'a str) {
        self.text = text;
    }

    /// Set the [style](struct.TextStyle.html) of the text.
    pub fn set_style(&mut self, style: TextStyle) {
        self.style = style;
    }
}

impl<'a> Drawable for Text<'a> {
    fn draw_on_canvas(&self, canvas: &mut Canvas) {
        let mut font = String::new();
        if self.style.italic {
            font.push_str("italic ");
        }
        if self.style.bold {
            font.push_str("bold ");
        }
        if self.style.underlined {
            unimplemented!("avoid underlined text for now");
        }
        font.push_str(&self.character_size.0.to_string());
        font.push_str(self.character_size.1);
        font.push_str(" '");
        font.push_str(&self.font.name);
        font.push_str("'");

        canvas.context.set_font(&font);
        canvas.fill_text(self.coords, self.text, None);
    }
}