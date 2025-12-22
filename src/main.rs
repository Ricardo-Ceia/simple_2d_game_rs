use macroquad::{prelude::*};

struct Particle{
    pos:    Vec2,
    velocity:   Vec2,
    life:   f32,
}

fn generate_enemy_pos(x_limit: f32) -> f32 {
    rand::gen_range(0.0, x_limit)  
}

fn draw_enemy(x: f32, y: f32,color:&Color){
    draw_rectangle(x, y, 60.0, 60.0, *color);
}

fn draw_missile(x: f32,y: f32){
    //Body
    draw_rectangle(x,y+10.0,10.0,20.0,LIGHTGRAY);

    //Nose
    draw_triangle(vec2(x,y+10.0), vec2(x+10.0,y+10.0), vec2(x+5.0,y), RED);

    //Fins
    draw_triangle(vec2(x, y + 20.0), vec2(x - 5.0, y + 30.0),vec2(x, y + 30.0), DARKGRAY);                                   
    draw_triangle(vec2(x + 10.0, y + 20.0), vec2(x + 15.0, y + 30.0), vec2(x + 10.0, y + 30.0), DARKGRAY);

    //Flame
    let flicker = rand::gen_range(0.0,5.0);
    draw_triangle(vec2(x + 2.0, y + 30.0), vec2(x + 8.0, y + 30.0), vec2(x + 5.0, y + 40.0 + flicker), ORANGE);              
    draw_triangle(vec2(x + 3.0, y + 30.0), vec2(x + 7.0, y + 30.0), vec2(x + 5.0, y + 35.0 + flicker), YELLOW);
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

fn generate_normal_point_in_circle(center:Vec2,max_radius:f32,std_dev:f32)->Vec2{
    loop{
        let u1:f32 = rand::gen_range(1e-6, 1.0);
        let u2 = rand::gen_range(0.0, 1.0);

        //Box-Muller transform
        //wiki link: https://en.wikipedia.org/wiki/Box%E2%80%93Muller_transform
        let magnitude = (-2.0 * u1.ln()).sqrt()*std_dev;
        let angle = 2.0*std::f32::consts::PI * u2;

        let x = magnitude*angle.cos();
        let y = magnitude*angle.sin();

        //Rejection Sampling
        //Only accept the point if it falls within the circle'std
        if x*x+y*y<=max_radius*max_radius{
            return vec2(center.x+x,center.y+y);
        }
    }    
}

fn create_explosion(center:Vec2,radius:f32,min_count:i32,max_count:i32)->Vec<Particle>{
    let count = rand::gen_range(min_count, max_count+1);
    let mut particles = Vec::with_capacity(count as usize);

    let std_dev =radius/3.0;

    for _ in 0..count {
        let start_pos = generate_normal_point_in_circle(center, radius, std_dev);
        let direction = (start_pos - center).normalize_or_zero();
        let speed = rand::gen_range(2.0, 5.0);

        particles.push(Particle { 
            pos: start_pos,
            velocity: direction*speed,
            life: 1.0 
        });
    }
    particles
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
    let mut particles: Vec<Particle> = Vec::new();

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
                    let impact_pos = vec2(missile.0 + 5.0,missile.1+rect_height/2.0);
                    let new_particles = create_explosion(impact_pos, 20.0, 10, 20);
                    particles.extend(new_particles);
                    missile.1= -1.0;
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

        for particle in particles.iter_mut(){
            particle.pos += particle.velocity;
            particle.life -= 0.02;

            let alpha = particle.life.clamp(0.0,1.0);
            draw_circle(particle.pos.x,particle.pos.y,2.0,Color::new(1.0,0.5,0.0,alpha));
        }
        
        particles.retain(|p| p.life>0.0);
        enemies.retain(|enemy| enemy.2>0);
        missiles.retain(|missile| missile.1>0.0);
        
        next_frame().await;
    }
}
