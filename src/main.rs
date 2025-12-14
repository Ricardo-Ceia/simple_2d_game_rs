use macroquad::prelude::*;

fn generate_enemy_pos(x_limit: f32) -> f32 {
    rand::gen_range(0.0, x_limit)  
}

#[macroquad::main("BasicShapes")]
async fn main() {
    
    let rect_width = 120.0;
    let rect_height = 10.0;
    let rect_y = screen_height() - 100.0;
    let speed = 3.0;

    let mut rect_pos_x = 0.0;
    let mut movement = 1.0;
    
    let mut enemy_pos_x;
    loop {
        let screen_w = screen_width();

        enemy_pos_x = generate_enemy_pos(screen_w);
        
        println!("Alien position->{enemy_pos_x}");

        rect_pos_x += speed*movement;

        if rect_pos_x+rect_width>=screen_w || rect_pos_x<0.0{
            movement*=-1.0;
        }
        
        draw_rectangle(rect_pos_x, rect_y, rect_width, rect_height, GREEN);
        draw_fps();
        next_frame().await;
    }
}
