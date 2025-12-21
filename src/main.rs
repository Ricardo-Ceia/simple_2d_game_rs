use macroquad::prelude::*;

fn generate_enemy_pos(x_limit: f32) -> f32 {
    rand::gen_range(0.0, x_limit)  
}

fn draw_enemy(enemy_position_x: f32, enemy_position_y: f32,color:&Color){
    draw_rectangle(enemy_position_x, enemy_position_y, 60.0, 60.0, *color);
}

fn draw_missile(missile_position_x: f32, missile_position_y: f32){
    draw_rectangle(missile_position_x, missile_position_y, 10.0, 30.0, YELLOW);
}

fn is_game_over(enemies: &[(f32,f32,i8,Color)],y_limit:f32)->bool{
    for (_,enemie_y,_,_) in enemies{
        if *enemie_y>=y_limit{
           return true; 
        }
    }
    false
}

fn check_colision(missile_x:f32,missile_y:f32,enemy_x:f32,enemy_y:f32)->bool{
    let missile_w = 10.0;
    let missile_h = 30.0;
    let enemy_w = 60.0;
    let enemy_h = 60.0;
    
    missile_x < enemy_x + enemy_w &&
    missile_x + missile_w > enemy_x &&
    missile_y < enemy_y + enemy_h &&
    missile_y + missile_h > enemy_y
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let rect_width = 120.0;
    let rect_height = 10.0;
    let rect_y = screen_height() - 100.0;
    let limit = rect_y-60.0;
    let mut rect_pos_x = 0.0;
    
    let mut frame_time = 0.0;
    
    let mut enemies:Vec<(f32,f32,i8,Color)> = Vec::new();
    let mut missiles: Vec<(f32, f32)> = Vec::new();

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

        if is_key_pressed(KeyCode::F){
            missiles.push((rect_pos_x+(rect_width/2.0),rect_y-rect_height*2.0))
        }

        if game_over{
            draw_text("GAME OVER", screen_w/2.0 - 100.0, screen_h/2.0, 50.0, RED);
            next_frame().await;
            continue;
        }

        if is_game_over(&enemies,limit){
            game_over = true;  
        }

        draw_rectangle(rect_pos_x, rect_y, rect_width, rect_height, GREEN);
        draw_fps(); 
        frame_time += get_frame_time();
        
        if frame_time > 4.0{
            let mut health:i8 = 3;
            enemies.push((generate_enemy_pos(screen_w),0.0,health,GREEN));
            frame_time = 0.0;
        }
        
        for missile in missiles.iter_mut(){
            for enemy in enemies.iter_mut(){
                if check_colision(missile.0, missile.1, enemy.0, enemy.1){
                    enemy.2-=1;
                    missile.1= -1.0;//Mark missile as dead 
                    break;
                }
            }
        }

        for (enemie_x,enemie_y,_,color) in &mut enemies{

            let progress = (*enemie_y/limit).clamp(0.0, 1.0);
            
            if progress<0.5{
                color.r = progress*2.0;
                color.g = 1.0;
            }else{
                color.r = 1.0;
                color.g = 1.0-(progress-0.5)*2.0;
            }
            color.b=0.0;

            draw_enemy(*enemie_x,*enemie_y,&color);
            *enemie_y+=0.1;
        }
        
        for (missile_x,missile_y) in &mut missiles{
            draw_missile(*missile_x,*missile_y);
            *missile_y-=1.0; 
        }

        enemies.retain(|enemy| enemy.2>0);
        missiles.retain(|missile| missile.1>0.0);
        
        next_frame().await;
    }
}
