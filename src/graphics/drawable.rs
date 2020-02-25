use super::canvas::Canvas;

pub trait Drawable {
    fn draw_on_canvas(&self, canvas: &mut Canvas);
}