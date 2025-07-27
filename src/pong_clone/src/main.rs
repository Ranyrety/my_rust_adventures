// src/pong_clone/src/main.rs
extern crate raylib;
use raylib::prelude::*;

#[derive(Default)]
struct Paddle {
    position: Vector2,
    size: Vector2,
}

#[derive(Default)]
struct Ball {
    position: Vector2,
    radius: f32,
    velocity: Vector2,
}
    // Initialize Raylib window
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Pong Clone")
        .build();

    rl.set_target_fps(60);

    let mut paddleR = Paddle {
        position : Vector2::new(780.0,10.0),
        size : Vector2::new(20.0,60.0),
    };

    let mut paddleL = Paddle {
        position : Vector2::new(0.0,10.0),
        size : Vector2::new(20.0,60.0),
    };
    
    let mut ball = Ball {
        position : Vector2::new(0.0,10.0),
        radius : 30.0,
        velocity : Vector2::new(2.0,0.1),
    };
    
    // Game loop
    while !rl.window_should_close() {
        // import for keyboard constants 
         use raylib::consts::KeyboardKey::*;
        // Update game logic here

        // Get player input and update paddleL position,
        if rl.is_key_down(KEY_DOWN) {
            paddleL.position.y += 5.0;
        }
        if rl.is_key_down(KEY_UP) {
            paddleL.position.y -= 5.0;
        } 
        if !rl.is_key_down(KEY_O) {
            ball.position += ball.velocity;
        }

        // Drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
       d.draw_rectangle(paddleR.position.x as i32,
           paddleR.position.y as i32,paddleR.size.x as i32,
           paddleR.size.y as i32, Color::RED);

        d.draw_rectangle(paddleL.position.x as i32,
           paddleL.position.y as i32,paddleL.size.x as i32,
           paddleL.size.y as i32, Color::GREEN);

        d.draw_circle_v(ball.position, ball.radius, Color::YELLOW);

        d.draw_text("Hello, Pong!", 12, 12, 20, Color::WHITE);
        // Draw game objects here
    }
}
