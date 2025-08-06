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
