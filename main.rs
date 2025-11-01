//This is the package for gaming library in rust
use macroquad::prelude::*;

//Calling the library through # and the folder
#[macroquad::main("My game")]

//Creating a async function to make a continous loop for the background 
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await    
    }
}