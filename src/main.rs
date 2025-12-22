use macroquad::{prelude::*};

struct Particle{
    pos:    Vec2,
    velocity:   Vec2,
    life:   f32,
}

fn generate_enemy_pos(x_limit: f32) -> f32 {
    rand::gen_range(0.0, x_limit)  
}

fn draw_terrain(screen_w: f32, screen_h: f32, ground_y: f32) {
    let tile_size = 20.0;
    let cols = (screen_w / tile_size).ceil() as i32;
    let rows = ((screen_h - ground_y) / tile_size).ceil() as i32;

    for r in 0..rows {
        for c in 0..cols {
            let x = c as f32 * tile_size;
            let y = ground_y + r as f32 * tile_size;
            
            // Top layer (Grass)
            if r == 0 {
                draw_rectangle(x, y, tile_size, tile_size, Color::new(0.13, 0.55, 0.13, 1.0)); // Base Grass
                // Detail: Lighter grass blades
                if (c + r) % 2 == 0 {
                    draw_rectangle(x + 5.0, y, 5.0, 5.0, Color::new(0.2, 0.7, 0.2, 1.0));
                }
                draw_rectangle(x, y + 15.0, tile_size, 5.0, Color::new(0.36, 0.25, 0.20, 1.0)); // Dirt transition
            } else {
                // Underground (Dirt/Stone)
                draw_rectangle(x, y, tile_size, tile_size, Color::new(0.36, 0.25, 0.20, 1.0)); // Brown Dirt
                // Detail: Stones
                if (c * r + c) % 7 == 0 {
                    draw_rectangle(x + 5.0, y + 5.0, 8.0, 6.0, Color::new(0.3, 0.2, 0.15, 1.0)); // Darker stone
                }
                if (c * r + r) % 5 == 0 {
                     draw_rectangle(x + 12.0, y + 12.0, 4.0, 4.0, Color::new(0.4, 0.3, 0.25, 1.0)); // Lighter speck
                }
            }
        }
    }
}

fn draw_hero(x: f32, y: f32) {
    let pixel_size = 1.5;
    
    // Expanded color palette for maximum detail
    // Hair colors - bright spiky anime style
    let hair_highlight = Color::new(1.0, 1.0, 0.4, 1.0);       // Brightest highlight
    let hair_bright = Color::new(1.0, 0.92, 0.1, 1.0);         // Bright yellow
    let hair_main = Color::new(1.0, 0.8, 0.0, 1.0);            // Main yellow
    let hair_mid = Color::new(0.9, 0.68, 0.0, 1.0);            // Mid tone
    let hair_dark = Color::new(0.7, 0.5, 0.0, 1.0);            // Dark shading
    let hair_shadow = Color::new(0.5, 0.32, 0.0, 1.0);         // Deep shadow
    let hair_deep = Color::new(0.35, 0.2, 0.0, 1.0);           // Deepest shadow
    
    // Skin tones
    let skin_highlight = Color::new(1.0, 0.88, 0.78, 1.0);     // Highlight
    let skin_light = Color::new(0.96, 0.8, 0.68, 1.0);         // Light skin
    let skin_main = Color::new(0.88, 0.7, 0.58, 1.0);          // Main skin
    let skin_mid = Color::new(0.78, 0.58, 0.48, 1.0);          // Mid tone
    let skin_shadow = Color::new(0.62, 0.42, 0.35, 1.0);       // Shadow
    let skin_dark = Color::new(0.48, 0.32, 0.28, 1.0);         // Dark shadow
    
    // Eyes - glowing red demonic
    let eye_glow = Color::new(1.0, 0.4, 0.3, 1.0);             // Glow around eye
    let eye_red = Color::new(0.95, 0.15, 0.1, 1.0);            // Red iris
    let eye_dark_red = Color::new(0.6, 0.05, 0.05, 1.0);       // Dark red
    let eye_pupil = Color::new(0.15, 0.0, 0.0, 1.0);           // Pupil
    let eyebrow = Color::new(0.55, 0.35, 0.0, 1.0);            // Eyebrow
    
    // Mouth
    let teeth = Color::new(1.0, 1.0, 0.98, 1.0);               // White teeth
    let mouth_dark = Color::new(0.25, 0.1, 0.1, 1.0);          // Mouth interior
    let lip = Color::new(0.7, 0.45, 0.4, 1.0);                 // Lip color
    
    // Cape - deep crimson red
    let cape_darkest = Color::new(0.2, 0.0, 0.05, 1.0);        // Darkest fold
    let cape_dark = Color::new(0.35, 0.02, 0.08, 1.0);         // Dark cape
    let cape_mid = Color::new(0.52, 0.05, 0.12, 1.0);          // Mid cape
    let cape_main = Color::new(0.68, 0.08, 0.15, 1.0);         // Main cape
    let cape_light = Color::new(0.82, 0.15, 0.2, 1.0);         // Light cape
    let cape_highlight = Color::new(0.95, 0.25, 0.28, 1.0);    // Highlight
    
    // Collar - high vampire collar
    let collar_dark = Color::new(0.4, 0.02, 0.08, 1.0);        // Dark collar
    let collar_main = Color::new(0.6, 0.05, 0.12, 1.0);        // Main collar
    let collar_light = Color::new(0.75, 0.1, 0.18, 1.0);       // Light collar
    
    // Outfit - black with subtle detail
    let outfit_darkest = Color::new(0.04, 0.04, 0.06, 1.0);    // Deepest black
    let outfit_dark = Color::new(0.1, 0.1, 0.12, 1.0);         // Dark
    let outfit_mid = Color::new(0.18, 0.18, 0.22, 1.0);        // Mid
    let outfit_light = Color::new(0.28, 0.28, 0.32, 1.0);      // Highlight
    
    // Bat symbol on chest
    let bat_dark = Color::new(0.45, 0.0, 0.06, 1.0);           // Dark bat
    let bat_main = Color::new(0.65, 0.02, 0.1, 1.0);           // Main bat
    let bat_light = Color::new(0.8, 0.08, 0.15, 1.0);          // Bat highlight
    
    // Sword - glowing cyan/teal magical blade
    let sword_glow = Color::new(0.85, 1.0, 1.0, 1.0);          // Brightest glow
    let sword_bright = Color::new(0.6, 0.95, 1.0, 1.0);        // Bright cyan
    let sword_main = Color::new(0.4, 0.8, 0.9, 1.0);           // Main blade
    let sword_mid = Color::new(0.25, 0.6, 0.75, 1.0);          // Mid blade
    let sword_dark = Color::new(0.15, 0.4, 0.55, 1.0);         // Edge
    let sword_edge = Color::new(0.1, 0.25, 0.35, 1.0);         // Dark edge
    
    // Handle - gold ornate
    let handle_bright = Color::new(1.0, 0.85, 0.3, 1.0);       // Bright gold
    let handle_main = Color::new(0.85, 0.65, 0.15, 1.0);       // Main gold
    let handle_dark = Color::new(0.6, 0.42, 0.08, 1.0);        // Dark gold
    let handle_shadow = Color::new(0.4, 0.28, 0.05, 1.0);      // Shadow
    
    let outline = Color::new(0.08, 0.04, 0.08, 1.0);           // Dark outline
    
    // 48x60 high-detail sprite
    let sprite: [[u8; 48]; 60] = [
        // Row 0-5: Top hair spikes
        [0,0,0,0,0,0,0,0,0,0,0,0,0,1,2,0,0,0,0,0,0,0,0,0,3,3,0,0,0,0,0,0,1,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,1,2,3,2,0,0,0,0,0,0,0,2,3,4,2,0,0,0,0,1,2,3,1,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,1,2,3,4,3,2,0,0,0,0,0,1,3,4,4,3,2,0,0,1,2,3,4,2,1,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,1,2,3,4,4,4,3,2,0,0,0,1,2,4,4,5,4,3,1,1,2,3,4,4,3,2,1,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,1,2,3,4,4,5,5,4,3,2,0,1,2,3,4,5,5,5,4,2,2,3,4,5,5,4,3,2,1,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,1,2,3,4,4,5,5,6,5,4,3,1,2,3,4,5,6,6,6,5,3,3,4,5,6,5,5,4,3,2,1,0,0,0,0,0,0,0,0,0],
        // Row 6-11: More spiky hair
        [0,0,0,0,0,0,0,1,2,3,4,5,5,6,6,7,6,5,4,2,3,4,5,6,7,7,7,6,4,4,5,6,6,6,5,5,4,3,2,1,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,1,2,3,4,5,5,6,7,7,7,7,6,5,3,4,5,6,7,7,7,7,7,5,5,6,7,7,6,6,5,5,4,3,2,1,0,0,0,0,0,0,0],
        [0,0,0,0,0,1,2,3,4,5,6,6,7,7,7,7,7,7,6,4,5,6,7,7,7,7,7,7,6,6,7,7,7,7,6,6,5,5,4,3,2,1,0,0,0,0,0,0],
        [0,0,0,0,1,2,3,4,5,6,6,7,7,7,4,5,7,7,7,5,6,7,7,7,5,4,7,7,7,7,7,7,5,7,7,7,6,6,5,4,3,2,1,0,0,0,0,0],
        [0,0,0,1,2,3,4,5,6,7,7,7,7,5,6,7,5,7,7,6,7,7,7,5,6,7,5,7,7,7,7,5,6,7,5,7,7,7,6,5,4,3,2,1,0,0,0,0],
        [0,0,1,2,3,4,5,6,7,7,7,7,5,6,7,7,6,5,7,7,7,7,5,6,7,7,6,5,7,7,5,6,7,7,6,5,7,7,7,6,5,4,3,2,1,0,0,0],
        // Row 12-17: Hair meets forehead
        [0,1,2,3,4,5,6,7,7,7,7,5,6,7,7,7,7,6,5,7,7,5,6,7,7,7,7,6,5,5,6,7,7,7,7,6,5,7,7,7,6,5,4,3,2,1,0,0],
        [0,2,3,4,5,6,7,7,7,7,5,6,7,7,6,5,6,7,6,5,5,6,7,7,6,5,6,7,6,6,7,7,6,5,6,7,6,5,7,7,7,6,5,4,3,2,0,0],
        [0,3,4,5,6,7,7,7,7,5,6,7,7,6,8,9,9,9,8,8,8,9,9,9,8,8,8,9,9,9,9,8,8,8,9,9,7,6,5,7,7,7,6,5,4,3,0,0],
        [0,4,5,6,7,7,7,7,5,6,7,6,8,9,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,9,8,6,6,5,7,7,7,6,5,4,0,0],
        [0,5,6,7,7,7,7,5,6,7,6,8,9,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,9,8,6,6,5,7,7,7,6,5,0,0],
        [0,0,6,7,7,7,5,6,7,6,8,9,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,9,8,6,6,5,7,7,6,0,0,0],
        // Row 18-23: Face with eyes
        [0,0,0,7,7,5,6,7,6,56,9,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,9,56,6,6,5,7,0,0,0,0],
        [0,0,0,0,5,6,7,6,56,9,10,15,15,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,15,15,10,10,10,9,56,6,6,5,0,0,0,0,0],
        [0,0,0,0,0,6,7,56,9,10,15,16,17,18,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,18,17,16,15,10,10,9,56,6,6,0,0,0,0,0,0],
        [0,0,0,0,0,56,7,9,10,10,15,17,18,19,11,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,11,19,18,17,15,10,10,10,9,7,56,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,56,9,10,12,10,10,12,11,10,10,10,10,10,12,10,10,12,10,10,10,10,10,10,10,10,11,12,10,10,12,10,10,9,56,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,56,9,10,12,12,10,10,10,10,10,10,10,10,10,12,12,10,10,10,10,10,10,10,10,10,10,10,10,12,12,10,10,9,56,0,0,0,0,0,0,0,0],
        // Row 24-29: Lower face, nose, mouth with grin
        [0,0,0,0,0,0,0,9,10,10,12,12,10,10,10,10,10,10,12,12,12,12,12,12,12,12,12,12,10,10,10,10,10,12,12,10,10,10,9,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,56,9,10,10,12,12,10,10,10,10,10,10,10,12,12,12,12,12,12,10,10,10,10,10,10,12,12,10,10,10,9,56,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,9,10,10,10,12,12,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,12,12,10,10,10,10,9,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,56,9,10,10,10,12,12,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,12,12,10,10,10,10,9,56,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,9,10,12,20,21,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,21,20,12,10,10,10,9,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,56,9,10,12,12,21,22,22,22,22,22,22,22,22,22,22,22,22,22,22,21,12,12,10,10,10,9,56,0,0,0,0,0,0,0,0,0,0,0],
        // Row 30-35: Chin, neck, cape collar
        [0,0,0,0,0,0,0,0,0,0,56,9,10,10,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,10,10,10,10,9,56,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,25,25,26,26,27,9,10,10,10,12,12,12,12,12,12,12,12,12,12,12,12,12,10,10,10,10,9,27,26,26,25,25,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,25,26,27,28,27,26,56,9,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,9,56,26,27,28,27,26,25,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,25,26,27,28,29,28,27,26,56,9,9,10,10,10,10,10,10,10,10,10,10,10,10,10,10,9,9,56,26,27,28,29,28,27,26,25,0,0,0,0,0,0,0,0],
        [0,0,0,25,26,27,28,29,30,29,28,27,26,30,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,30,26,27,28,29,30,29,28,27,26,25,0,0,0,0,0,0,0],
        [0,0,25,26,27,28,29,30,29,28,27,26,30,31,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,31,30,26,27,28,29,30,29,28,27,26,25,0,0,0,0,0,0],
        // Row 36-41: Cape and torso with bat symbol
        [0,25,26,27,28,29,30,29,28,27,26,30,31,32,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,32,31,30,26,27,28,29,30,29,28,27,26,25,0,0,0,0,0],
        [0,26,27,28,29,30,29,28,27,26,30,31,32,33,33,34,35,35,35,35,35,35,35,35,35,35,35,35,34,33,33,32,31,30,26,27,28,29,30,29,28,27,26,0,45,0,0,0],
        [0,26,27,28,29,30,29,28,27,30,31,32,33,33,34,35,36,36,35,36,36,36,36,36,36,35,36,36,35,34,33,33,32,31,30,27,28,29,30,29,28,27,26,44,45,46,0,0],
        [0,26,27,28,29,30,29,28,27,30,31,32,33,34,35,36,36,35,34,35,35,35,35,35,35,34,35,36,36,35,34,33,32,31,30,27,28,29,30,29,28,27,26,44,45,46,0,0],
        [0,25,26,27,28,29,30,29,28,30,31,32,33,34,35,36,35,34,33,34,34,34,34,34,34,33,34,35,36,35,34,33,32,31,30,28,29,30,29,28,27,26,25,44,45,46,47,0],
        [0,0,25,26,27,28,29,30,29,30,31,32,33,34,35,35,34,33,32,33,33,33,33,33,33,32,33,34,35,35,34,33,32,31,30,29,30,29,28,27,26,25,43,44,45,46,47,0],
        // Row 42-47: Lower cape and torso
        [0,0,0,25,26,27,28,29,30,30,31,32,33,33,34,34,33,32,31,32,32,32,32,32,32,31,32,33,34,34,33,33,32,31,30,30,29,28,27,26,25,0,43,44,45,46,0,0],
        [0,0,0,0,25,26,27,28,29,30,31,32,32,33,33,33,32,31,30,31,31,31,31,31,31,30,31,32,33,33,33,32,32,31,30,29,28,27,26,25,0,0,43,44,45,46,0,0],
        [0,0,0,0,0,25,26,27,28,29,30,31,32,32,32,32,31,30,30,30,30,30,30,30,30,30,30,31,32,32,32,32,31,30,29,28,27,26,25,0,0,42,43,44,45,0,0,0],
        [0,0,0,0,0,0,25,26,27,28,29,30,31,31,31,31,30,30,0,0,0,0,0,0,0,0,30,30,31,31,31,31,30,29,28,27,26,25,0,0,42,43,44,45,46,0,0,0],
        [0,0,0,0,0,0,0,25,26,27,28,29,30,30,30,30,30,0,0,0,0,0,0,0,0,0,0,30,30,30,30,30,29,28,27,26,25,0,0,0,42,43,44,45,0,0,0,0],
        [0,0,0,0,0,0,0,0,25,26,27,28,30,30,30,30,0,0,0,0,0,0,0,0,0,0,0,0,30,30,30,30,28,27,26,25,0,0,0,42,43,44,45,46,0,0,0,0],
        // Row 48-53: Legs
        [0,0,0,0,0,0,0,0,0,25,26,27,30,30,31,30,0,0,0,0,0,0,0,0,0,0,0,0,30,31,30,30,27,26,25,0,0,0,0,42,43,44,45,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,25,26,30,30,31,31,30,0,0,0,0,0,0,0,0,0,0,30,31,31,30,30,26,25,0,0,0,0,42,43,44,45,46,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,25,30,31,31,31,30,0,0,0,0,0,0,0,0,0,0,30,31,31,31,30,25,0,0,0,0,0,42,43,44,45,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,30,31,31,31,30,0,0,0,0,0,0,0,0,0,0,30,31,31,31,30,0,0,0,0,0,42,43,44,45,46,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,30,31,31,31,30,0,0,0,0,0,0,0,0,0,0,30,31,31,31,30,0,0,0,0,0,42,43,44,45,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,30,30,31,31,30,0,0,0,0,0,0,0,0,0,0,30,31,31,30,30,0,0,0,0,42,43,44,45,46,0,0,0,0,0,0,0],
        // Row 54-59: Feet and sword tip
        [0,0,0,0,0,0,0,0,0,0,0,0,30,30,31,31,30,0,0,0,0,0,0,0,0,0,0,30,31,31,30,30,0,0,0,0,42,43,44,45,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,30,30,30,30,31,30,0,0,0,0,0,0,0,0,0,0,30,31,30,30,30,30,0,0,42,43,44,45,46,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,30,30,31,30,30,0,0,0,0,0,0,0,0,0,0,0,0,30,30,31,30,30,0,0,42,43,44,45,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,30,31,31,30,0,0,0,0,0,0,0,0,0,0,0,0,0,0,30,31,31,30,0,0,42,43,44,45,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,30,31,30,30,0,0,0,0,0,0,0,0,0,0,0,0,0,0,30,30,31,30,0,0,0,43,44,45,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,30,30,30,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,30,30,30,0,0,0,0,44,45,0,0,0,0,0,0,0,0,0],
    ];
    
    for (row_i, row) in sprite.iter().enumerate() {
        for (col_i, cell) in row.iter().enumerate() {
            let color = match cell {
                // Hair
                1 => hair_highlight,
                2 => hair_bright,
                3 => hair_main,
                4 => hair_mid,
                5 => hair_dark,
                6 => hair_shadow,
                7 => hair_deep,
                // Skin
                8 => skin_highlight,
                9 => skin_light,
                10 => skin_main,
                11 => skin_mid,
                12 => skin_shadow,
                13 => skin_dark,
                // Eyes
                15 => eyebrow,
                16 => eye_glow,
                17 => eye_red,
                18 => eye_dark_red,
                19 => eye_pupil,
                // Mouth
                20 => lip,
                21 => mouth_dark,
                22 => teeth,
                // Cape
                25 => cape_darkest,
                26 => cape_dark,
                27 => cape_mid,
                28 => cape_main,
                29 => cape_light,
                // Outfit
                30 => outfit_darkest,
                31 => outfit_dark,
                32 => outfit_mid,
                33 => outfit_light,
                // Bat symbol
                34 => bat_dark,
                35 => bat_main,
                36 => bat_light,
                // Handle
                48 => handle_shadow,
                49 => handle_dark,
                50 => handle_main,
                51 => handle_bright,
                // Outline
                56 => outline,
                _ => continue,
            };
            draw_rectangle(
                x + col_i as f32 * pixel_size,
                y + row_i as f32 * pixel_size,
                pixel_size,
                pixel_size,
                color
            );
        }
    }
}

fn draw_enemy(x: f32, y: f32, color: &Color) {
    let pixel_size = 4.0;
    // 16x16 Skeleton Sprite
    let sprite = [
        [0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0], // Skull
        [0,0,0,0,1,1,1,1,1,1,1,0,0,0,0,0],
        [0,0,0,0,1,3,1,1,1,3,1,0,0,0,0,0], // Eyes
        [0,0,0,0,1,1,1,1,1,1,1,0,0,0,0,0],
        [0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0],
        [0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0], // Neck
        [0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0], // Shoulders
        [0,1,1,0,0,1,1,1,1,1,0,0,1,1,0,0], // Ribs
        [0,1,0,0,0,1,0,1,0,1,0,0,0,1,0,0],
        [0,1,0,0,0,1,1,1,1,1,0,0,0,1,0,0],
        [0,1,0,0,0,0,1,1,1,0,0,0,0,1,0,0], // Pelvis
        [0,0,0,0,0,1,1,0,1,1,0,0,0,0,0,0],
        [0,0,0,0,1,1,0,0,0,1,1,0,0,0,0,0], // Legs
        [0,0,0,0,1,1,0,0,0,1,1,0,0,0,0,0],
        [0,0,0,0,1,1,0,0,0,1,1,0,0,0,0,0],
        [0,0,0,1,1,0,0,0,0,0,1,1,0,0,0,0],
    ];
    
    for (row_i, row) in sprite.iter().enumerate() {
        for (col_i, cell) in row.iter().enumerate() {
            let base_color = match cell {
                1 => Color::new(0.9, 0.9, 0.85, 1.0), // Bone White
                3 => BLACK, // Eye sockets
                _ => continue,
            };
            
            // Apply tint from the 'color' parameter (which contains damage redness)
            let final_color = Color::new(
                base_color.r * color.r,
                base_color.g * color.g,
                base_color.b * color.b,
                1.0
            );

            draw_rectangle(x + col_i as f32 * pixel_size, y + row_i as f32 * pixel_size, pixel_size, pixel_size, final_color);
        }
    }
}

fn draw_projectile(x: f32, y: f32){
    let pixel_size = 3.0;
    // Blood Magic Orb
    // Core
    draw_circle(x + 5.0, y + 5.0, 6.0, RED);
    draw_circle(x + 5.0, y + 5.0, 4.0, Color::new(1.0, 0.2, 0.2, 1.0)); // Lighter red
    draw_circle(x + 5.0, y + 5.0, 2.0, WHITE); // Highlight
    
    // Trail particles (simulated)
    let flicker = rand::gen_range(0.0, 5.0);
    draw_circle(x + 5.0, y + 12.0 + flicker, 3.0, Color::new(0.8, 0.0, 0.0, 0.7));
    draw_circle(x + 5.0, y + 18.0 + flicker, 2.0, Color::new(0.6, 0.0, 0.0, 0.5));
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
    let missile_w = 8.0;
    let missile_h = 8.0;
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
    let rect_width = 65.0;
    let rect_y = screen_height() - 100.0;
    let limit = rect_y-60.0;
    let mut rect_pos_x = 0.0;
    
    let mut frame_time = 0.0;
    
    let mut enemies:Vec<(f32,f32,i8,Color)> = Vec::new();
    let mut missiles: Vec<(f32, f32)> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();

    let mut game_over:bool = false;

    let mut stars: Vec<(f32, f32, f32)> = Vec::new();
    for _ in 0..100 {
        stars.push((
            rand::gen_range(0.0, screen_width()),
            rand::gen_range(0.0, screen_height() / 2.0),
            rand::gen_range(1.0, 3.0),
        ));
    }

    loop {
        
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Draw Background (Gradient Sky)
        let steps = 20;
        let step_h = screen_h / steps as f32;
        for i in 0..steps {
            let t = i as f32 / steps as f32;
            // Gradient from Dark Blue to Purple-ish
            let color = Color::new(0.05 + 0.05*t, 0.05, 0.2 - 0.1*t, 1.0);
            draw_rectangle(0.0, i as f32 * step_h, screen_w, step_h, color);
        }

        // Draw Stars
        for (x, y, size) in &stars {
            draw_circle(*x, *y, *size, Color::new(1.0, 1.0, 1.0, 0.8)); // Slightly transparent
        }

        // Draw Moon (Glowing)
        draw_circle(screen_w - 80.0, 80.0, 42.0, Color::new(1.0, 1.0, 0.8, 0.2)); // Glow
        draw_circle(screen_w - 80.0, 80.0, 40.0, LIGHTGRAY);
        draw_circle(screen_w - 95.0, 80.0, 35.0, Color::new(0.05, 0.05, 0.2, 1.0)); // Crescent effect (match sky roughly)

        // Draw Terrain (Textured)
        let ground_y = rect_y + 60.0;
        draw_terrain(screen_w, screen_h, ground_y);

        if is_key_down(KeyCode::Right) && rect_pos_x<=screen_w-rect_width{
            rect_pos_x += 1.0;
        }

        if is_key_down(KeyCode::Left) && rect_pos_x>=0.0{
            rect_pos_x -= 1.0;
        }

        if is_key_pressed(KeyCode::F){
            missiles.push((rect_pos_x+(rect_width/2.0),rect_y - 20.0))
        }

        if game_over{
            draw_text("GAME OVER", screen_w/2.0 - 100.0, screen_h/2.0, 50.0, RED);
            next_frame().await;
            continue;
        }

        if is_game_over(&enemies,limit){
            game_over = true;  
        }
        
        draw_hero(rect_pos_x,rect_y);
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
                    let impact_pos = vec2(missile.0 + 5.0,missile.1+15.0);
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
            draw_projectile(*missile_x,*missile_y);
            *missile_y-=2.0; // Faster magic bolt
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
