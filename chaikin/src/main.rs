mod app;
use app::*;
use macroquad::prelude::*;

#[macroquad::main("Chaikin's Algorithm")]
async fn main() {

    let mut app: App = App::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        draw_ui(&app);
    
        next_frame().await;
        }

     
}
