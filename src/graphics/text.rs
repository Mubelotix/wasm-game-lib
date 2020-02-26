use super::font::Font;
use crate::graphics::drawable::Drawable;
use crate::graphics::canvas::Canvas;

const EMPTY_STR: &str = "";
const PX_STR: &str = "px";

pub struct TextStyle {
    italic: bool,
    bold: bool,
    underlined: bool,
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

pub struct Text<'a> {
    pub coords: (usize, usize),
    pub font: &'a Font,
    pub text: &'a str,
    pub style: TextStyle,
    pub character_size: (usize, &'a str)
}

impl<'a> Text<'a> {
    pub fn new(font: &'a Font) -> Text<'a> {
        Text {
            coords: (0,0),
            font,
            text: EMPTY_STR,
            style: TextStyle::default(),
            character_size: (26, PX_STR)
        }
    }

    pub fn new_with_text_and_coords(font: &'a Font, text: &'a str, coords: (usize, usize)) -> Text<'a> {
        Text {
            coords,
            font,
            text,
            style: TextStyle::default(),
            character_size: (26, PX_STR)
        }
    }

    pub fn new_with_options(font: &'a Font, text: &'a str, coords: (usize, usize), style: TextStyle, character_size: (usize, &'a str)) -> Text<'a> {
        Text {
            coords,
            font,
            text,
            style,
            character_size
        }
    }

    pub fn set_text(&mut self, text: &'a str) {
        self.text = text;
    }

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