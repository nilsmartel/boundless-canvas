use ultraviolet::{Rotor2, Vec2 };

fn main() {
    println!("Hello, world!");
}

/// Represents some aligned canvas in an actual window
struct CanvasWindow {
    offset: Vec2,
    rotation: f32,
    scale: f32,

    canvas: Canvas,
}

impl CanvasWindow {

    fn translate_to_canvas_space(&self, screen_position: Vec2) -> Vec2 {
        // the canvas is offseted around it's top right corner.
        let position = screen_position - self.offset;

        // the canvas is rotated around it's top right corner.
        let position = position.rotated_by(Rotor2::from_angle(-self.rotation));

        // last normalize the scale
        let position = position * self.scale;

        position
    }
}

struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

#[derive(Copy, Clone)]
struct Color(u8, u8, u8, u8);
