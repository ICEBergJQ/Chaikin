use std::{thread, time};

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

    fn chaikin_alg(&mut self) {
        let points = &self.chaikin_points;

        let length = points.len();
        let start = points[0];
        let end = points[length - 1];

        let mut new_points = vec![start];

        for i in 0..length - 1 {
            let current = points[i];
            let next = points[i + 1];
            let dx = next.x - current.x;
            let dy = next.y - current.y;

            let new_1 = Point::new(current.x + dx * 0.25, current.y + dy * 0.25);
            let new_2 = Point::new(current.x + dx * 0.75, current.y + dy * 0.75);

            new_points.push(new_1);
            new_points.push(new_2);
        }

        new_points.push(end);

        self.chaikin_points = new_points;
    }

    pub fn animate(&mut self) {
        if self.chaikin_points.len() == 2 {
            let start = self.chaikin_points[0];
            let end = self.chaikin_points[1];
            draw_line(start.x, start.y, end.x, end.y, 2.0, WHITE);
        } else if self.chaikin_points.len() > 2 {
            for i in 0..self.chaikin_points.len() - 1 {
                let start = self.chaikin_points[i];
                let end  = self.chaikin_points[i + 1];
                draw_line(start.x, start.y, end.x, end.y, 2.0, WHITE);
            }

            thread::sleep(time::Duration::from_millis(500));

            self.chaikin_alg();
            
            self.steps += 1;

            if self.steps == 7 {
                self.chaikin_points = self.default_points.clone();
                self.steps = 0;
            }
        } else {
            self.start_animation = false;
        }
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