// src/pong_clone/src/main.rs
use raylib::prelude::*;

fn main() {
    // Initialize Raylib window
    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Pong Clone")
        .build();

    rl.set_target_fps(60);

    // Game loop
    while !rl.window_should_close() {
        // Update game logic here

        // Drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("Hello, Pong!", 12, 12, 20, Color::WHITE);
        // Draw game objects here
    }
}
