//This is the package for gaming library in rust
use macroquad::prelude::*;

struct Shape {
        size: f32,
        speed: f32,
        x: f32,
        y: f32,
    }
//Calling the library through # and the folder
#[macroquad::main("My game")]

//Creating a async function to make a continous loop for the background 
async fn main() {
    
    //Sticking the movement speed of the game
    const MOVEMENT_SPEED: f32 = 200.0;
    
    //Passing a random number generator for the object that will be in the pixel way
    rand::srand(miniquad::date::now() as u64);
    let mut squares = vec![];
    
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };
    
    loop {
        clear_background(DARKPURPLE);
        
        //Capturing the time for the pixel
        let delta_time = get_frame_time();

        // On the keyboard capturing the movement from whatever was press
        if is_key_down(KeyCode::Right) {
            circle.x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= MOVEMENT_SPEED * delta_time;
        }

        //Clamp X and Y to be within the screen
        circle.x = clamp(circle.x, 0.0, screen_width());
        circle.y = clamp(circle.y, 0.0, screen_height());

        //Generating squares
        if rand::gen_range(0, 99) >= 95  {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
            });
        }
        //Move Squares 
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }

        // Remove squares below bottom of screen
        squares.retain(|square| square.y < screen_height() + square.size);

        //Drawing everything
        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }
        next_frame().await    
    }
}