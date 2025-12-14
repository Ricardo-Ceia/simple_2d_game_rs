use macroquad::{miniquad::native::linux_x11::libx11::Screen, prelude::*};

#[macroquad::main("BasicShapes")]
async fn main() {
    
    let rect_width = 120.0;
    let rect_height = 60.0;
    let rect_y = 100.0;
    let speed = 3.0;

    let mut rect_pos_x = 0.0;
    let mut movement = 1.0;

    loop {
        let screen_w = screen_width();

        rect_pos_x += speed*movement;

        if rect_pos_x+rect_width>=screen_w || rect_pos_x<0.0{
            movement*=-1.0;
        }
        
        draw_rectangle(rect_pos_x, rect_y, rect_width, rect_height, GREEN);

        next_frame().await;
    }
}
