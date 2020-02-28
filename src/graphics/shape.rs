use super::drawable::Drawable;
use super::color::Color;
use super::canvas::*;
use wasm_bindgen::JsValue;

/// A drawable rectangle
pub struct Rectangle {
    /// the [style](../canvas/struct.LineStyle.html) of the border
    pub line_style: LineStyle,
    /// point y and point y (in pixels)
    pub top_left: (f64, f64),
    /// width and height (in pixels)
    pub dimensions: (f64, f64),
    /// if some, the square will be filled by this color
    pub fill_color: Option<Color>
}

impl Rectangle {
    /// Create a rectangle from the top left and the bottom right point
    pub fn new_with_two_points(point_a: (f64, f64), point_b: (f64, f64)) -> Rectangle {
        Rectangle {
            line_style: LineStyle::default(),
            top_left: point_a,
            dimensions: (point_b.0 - point_a.0, point_b.1 - point_a.1),
            fill_color: None
        }
    }

    /// Create a rectangle from the top left point and a dimension
    pub fn new_with_dimension(point: (f64, f64), dimensions: (f64, f64)) -> Rectangle {
        Rectangle {
            line_style: LineStyle::default(),
            top_left: point,
            dimensions: dimensions,
            fill_color: None
        }
    }
}

impl Drawable for Rectangle {
    fn draw_on_canvas(&self, mut canvas: &mut Canvas) {
        canvas.context.begin_path();
        canvas.context.rect(self.top_left.0, self.top_left.1, self.dimensions.0, self.dimensions.1);
        if let Some(color) = &self.fill_color {
            canvas.context.set_fill_style(&JsValue::from_str(&color.to_string()));
            canvas.context.fill();
        }
        self.line_style.apply_on_canvas(&mut canvas);
        canvas.context.stroke();
    }
}

/// A simple drawable line
pub struct Line {
    /// the [style](../canvas/struct.LineStyle.html) of the line
    pub line_style: LineStyle,
    #[allow(missing_docs)]
    pub point_a: (f64, f64),
    #[allow(missing_docs)]
    pub point_b: (f64, f64)
}

impl Line {
    /// Create a line with a [default style](../canvas/struct.LineStyle.html#method.default)
    pub fn new(point_a: (f64, f64), point_b: (f64, f64)) -> Line {
        Line {
            line_style: LineStyle::default(),
            point_a,
            point_b
        }
    }
}

impl Drawable for Line {
    fn draw_on_canvas(&self, mut canvas: &mut Canvas) {
        canvas.context.begin_path();
        canvas.context.move_to(self.point_a.0, self.point_a.1);
        canvas.context.line_to(self.point_b.0, self.point_b.1);
        self.line_style.apply_on_canvas(&mut canvas);
        canvas.context.stroke();
    }
}