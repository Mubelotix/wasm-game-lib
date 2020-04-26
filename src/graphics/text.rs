use super::font::Font;
use crate::graphics::drawable::Drawable;
use crate::graphics::canvas::Canvas;
use crate::graphics::color::Color;
use wasm_bindgen::JsValue;

const PX_STR: &str = "px";

/// A struct representing the style of a [Text](struct.Text.html).
pub struct TextStyle {
    /// Italic
    pub italic: bool,
    /// Bold
    pub bold: bool,
    /// Underlined is not supported for now!
    pub underlined: bool,
    /// The color of the text
    pub color: Color
}

impl Default for TextStyle {
    fn default() -> TextStyle {
        TextStyle {
            italic: false,
            bold: false,
            underlined: false,
            color: Color::black()
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
    pub text: String,
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
            text: String::new(),
            style: TextStyle::default(),
            character_size: (26, PX_STR)
        }
    }

    /// Create a new text with some default values.
    pub fn new_with_text_and_coords(font: &'a Font, text: String, coords: (usize, usize)) -> Text<'a> {
        Text {
            coords,
            font,
            text,
            style: TextStyle::default(),
            character_size: (26, PX_STR)
        }
    }

    /// Create a new text with no default value.
    pub fn new_with_options(font: &'a Font, text: String, coords: (usize, usize), style: TextStyle, character_size: (usize, &'a str)) -> Text<'a> {
        Text {
            coords,
            font,
            text,
            style,
            character_size
        }
    }

    /// Set the displayed text.
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    /// Set the [style](struct.TextStyle.html) of the text.
    pub fn set_style(&mut self, style: TextStyle) {
        self.style = style;
    }

    /// Get the width of the text
    /// Needs a mutable reference to a canvas
    pub fn get_width(&self, mut canvas: &mut Canvas) -> f64 {
        self.apply_style_on_canvas(&mut canvas);
        canvas.context.measure_text(&self.text).unwrap().width()
    }

    fn apply_style_on_canvas(&self, canvas: &mut Canvas) {
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
        canvas.context.set_fill_style(&JsValue::from_str(&self.style.color.to_string()));
    }
}

impl<'a> Drawable for Text<'a> {
    fn draw_on_canvas(&self, mut canvas: &mut Canvas) {
        self.apply_style_on_canvas(&mut canvas);
        for (idx, text) in self.text.split('\n').enumerate() {
            let mut coords = self.coords;
            coords.1 += self.character_size.0 * idx;
            canvas.fill_text(coords, text, None);
        }
    }
}