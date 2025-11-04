//This is the package for gaming library in rust
use macroquad::prelude::*;

//Creating the outline for each variable that will be added to the game 
struct Shape {
        size: f32,
        speed: f32,
        x: f32,
        y: f32,
        collided: bool,
    }

//Creating the shape for the rectangle and specifying size
impl Shape {
        fn collides_with(&self, other: &Self) -> bool {
            self.rect().overlaps(&other.rect())
        }
        fn rect(&self) -> Rect {
            Rect {
                x: self.x - self.size / 2.0,
                y: self.y - self.size / 2.0,
                w: self.size,
                h: self.size,
            }
        }
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
    
    //Creating the bullets for the game 
    let mut bullets: Vec<Shape> = vec![];

    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
    };
    let mut gameover = false;

    loop {
        clear_background(DARKPURPLE);
        
       if !gameover {
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
        if is_key_pressed(KeyCode::Space) {
            bullets.push(Shape {
                x: circle.x,
                y: circle.y,
                speed: circle.speed * 2.0,
                size: 5.0,
                collided: false,
            });
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
                collided: false,
            });
        }
        //Move Squares 
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }
        for bullet in &mut bullets {
            bullet.y -= bullet.speed * delta_time;
        }
    }
        // Remove squares below bottom of screen
        squares.retain(|square| square.y < screen_height() + square.size);
        bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);


        //Remove collided shapes
        squares.retain(|square| !square.collided);
        bullets.retain(|bullet| !bullet.collided);



        if squares.iter().any(|square| circle.collides_with(square)){
            gameover = true;
        }

        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.collides_with(square) {
                    bullet.collided = true;
                    square.collided = true;
                }
            }
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }

        //Drawing everything

        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 1.0, RED);
        }

        draw_circle(circle.x, circle.y, circle.size / 3.0, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }
        if gameover {
            let text = "Try Again";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text (
                text,
                screen_width() / 2.0 -text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }
        next_frame().await    
    }
}