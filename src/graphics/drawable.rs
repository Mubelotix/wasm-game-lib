use super::canvas::Canvas;

/// This trait allows an object to be drawn on a canvas like this:
/// 
/// ```ignore
/// canvas.draw(&object);
/// ```
pub trait Drawable {
    /// This method is called by the [draw method](../canvas/struct.Canvas.html#method.draw).
    fn draw_on_canvas(&self, canvas: &mut Canvas);
}