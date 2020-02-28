use super::canvas::Canvas;

/// This trait allows an object to be drawn on a canvas like this:
/// 
/// ```ignore
/// canvas.draw(&object);
/// ```
/// 
/// If you are experienced with [html5 canvas element](https://www.html5canvastutorials.com/) you can get the
/// [WebSys canvas object](https://docs.rs/web-sys/0.3.35/web_sys/struct.CanvasRenderingContext2d.html) and draw on it directly.
pub trait Drawable {
    /// This method is called by the [draw method](../canvas/struct.Canvas.html#method.draw).
    fn draw_on_canvas(&self, canvas: &mut Canvas);
}