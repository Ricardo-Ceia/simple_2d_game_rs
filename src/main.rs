use macroquad::prelude::*;

fn generate_enemy_pos(x_limit: f32) -> f32 {
    rand::gen_range(0.0, x_limit)  
}

fn draw_enemy(enemy_position_x: f32, enemy_position_y: f32,color:&Color){
    draw_rectangle(enemy_position_x, enemy_position_y, 60.0, 60.0, *color);
}

fn is_game_over(enemies: &[(f32,f32,Color)],y_limit:f32)->bool{
    for (_,enemie_y,_) in enemies{
        if *enemie_y>=y_limit{
           return true; 
        }
    }
    false
}


#[macroquad::main("BasicShapes")]
async fn main() {
    let rect_width = 120.0;
    let rect_height = 10.0;
    let rect_y = screen_height() - 100.0;

    let mut rect_pos_x = 0.0;
    
    let mut frame_time = 0.0;
    let mut enemies:Vec<(f32,f32,Color)> = Vec::new();
    let mut game_over:bool = false;
    loop {
        
        let screen_w = screen_width();
        let screen_h = screen_height();

        if is_key_down(KeyCode::Right) && rect_pos_x<=screen_w-rect_width{
            rect_pos_x += 1.0;
        }

        if is_key_down(KeyCode::Left) && rect_pos_x>=0.0{
            rect_pos_x -= 1.0;
        }

        if game_over{
            draw_text("GAME OVER", screen_w/2.0 - 100.0, screen_h/2.0, 50.0, RED);
            next_frame().await;
            continue;
        }

        if is_game_over(&enemies,rect_y-60.0){
            game_over = true;  
        }

        draw_rectangle(rect_pos_x, rect_y, rect_width, rect_height, GREEN);
        draw_fps(); 
        frame_time += get_frame_time();
        
        if frame_time > 4.0{
            enemies.push((generate_enemy_pos(screen_w),0.0,GREEN));
            frame_time = 0.0;
        }

        for (enemie_x,enemie_y,color) in &mut enemies{
            if color.r<1.0{
                color.r+=0.002;
            }
            if color.g>0.0{
                color.g-=0.002;
            }
            draw_enemy(*enemie_x,*enemie_y,&color);
            *enemie_y+=1.;
        }

        next_frame().await;
    }
}
