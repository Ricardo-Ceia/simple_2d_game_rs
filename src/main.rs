use macroquad::prelude::*;

fn generate_enemy_pos(x_limit: f32) -> f32 {
    rand::gen_range(0.0, x_limit)  
}

fn draw_enemy(enemy_position_x: f32, enemy_position_y: f32){
    draw_rectangle(enemy_position_x, enemy_position_y, 60.0, 60.0, GREEN);
}

#[macroquad::main("BasicShapes")]
async fn main() {
         
    let rect_width = 120.0;
    let rect_height = 10.0;
    let rect_y = screen_height() - 100.0;
    let speed = 3.0;

    let mut rect_pos_x = 0.0;
    let mut movement = 1.0;
    
    let mut frame_time = 0.0;
    let mut enemies:Vec<(f32,f32)> = Vec::new(); 
    loop {

        let screen_w = screen_width();

        rect_pos_x += speed*movement;

        if rect_pos_x+rect_width>=screen_w || rect_pos_x<0.0{
            movement*=-1.0;
        }
        
        draw_rectangle(rect_pos_x, rect_y, rect_width, rect_height, GREEN);
        draw_fps(); 
        frame_time += get_frame_time();
        if frame_time > 4.0{
            enemies.push((generate_enemy_pos(screen_w),0.0));
            frame_time = 0.0;
        }

        for (enemie_x,enemie_y) in &mut enemies{
            draw_enemy(*enemie_x,*enemie_y);
            *enemie_y+=0.1;
        }
        next_frame().await;
    }
}
