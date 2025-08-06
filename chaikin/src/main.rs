use macroquad::prelude::*;

#[macroquad::main("Interactive Points Example")]
async fn main() {
        draw_text(
            "Left click to add points | R to reset | ESC to exit",
            20.0,
            20.0,
            25.0,
            GREEN,
        );
}
