use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}


impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }   
}

pub struct App {
    pub default_points: Vec<Point>,
    pub chaikin_points: Vec<Point>,
    pub start_animation: bool,
    pub steps: u32,
}

impl App {
    pub fn new() -> Self {
          Self {
            default_points: Vec::new(),
            chaikin_points: Vec::new(),
            start_animation: false,
            steps: 0,
        }
    }

     pub fn new_point(&mut self, x: f32, y: f32) {
        self.default_points.push(Point::new(x, y))
    }

    pub fn reset(&mut self) {
        *self = App::new();
    }
}


pub fn draw_ui(app: &App) {
      if !app.start_animation{
            draw_text(
                "Left click to add points | R to reset | ESC to exit",
                20.0,
                20.0,
                25.0,
                GREEN,
            );
        }

        draw_text(
            &format!("init Points: {}", app.default_points.len()),
            20.0,
            50.0,
            20.0,
            WHITE,
        );
        draw_text(&format!("steps: {}", app.steps), 20.0, 80.0, 20.0, WHITE);
}