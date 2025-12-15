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
    let mut frame_time = 0.0;

    loop {

        let screen_w = screen_width();

        rect_pos_x += speed*movement;

        if rect_pos_x+rect_width>=screen_w || rect_pos_x<0.0{
            movement*=-1.0;
        }
        
        draw_rectangle(rect_pos_x, rect_y, rect_width, rect_height, GREEN);
        draw_fps(); 
        frame_time += get_frame_time();
        //TODO:here verify how should the period of spanning enemies be created
        if frame_time > (1000.0*get_frame_time()){
            enemy_pos_x = generate_enemy_pos(screen_w); 
            println!("Enemy spanning at position->{enemy_pos_x} time->{frame_time}");
            frame_time = 0.0;
        }
        next_frame().await;
    }
}
