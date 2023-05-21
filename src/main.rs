use macroquad::prelude::*;
mod field;
use self::field::{Field, SimulationParams};



#[macroquad::main("Physarum (Slime Mold) Simulation")]
async fn main() {
    request_new_screen_size(640., 385.);

    let (field_w, field_h) = (1280u16, 720u16);
    let n_agents = 150000;
    // let (field_w, field_h) = (3840u16, 2160u16);
    let mut field = Field::new(field_w, field_h, 
    SimulationParams {
        evaporation_speed: 0.25,
        diffuse_speed: 5.0,
        turn_speed: 100.0,
        sense_angle_difference: 2.0,
        sense_distance: 15.0,
        sense_size: 3,
        move_speed: 25.0,
        // pheromone_color: Color::from_rgba(57, 173, 227, 255),
        // background_color: Color::from_rgba(48, 27, 117, 255),
        ..SimulationParams::default()
    });
    field.add_inward_circle(n_agents, 300.0);

    let mut img = Image::gen_image_color(field_w, field_h, BLACK);
    let texture = Texture2D::from_image(&img);
    let font = load_ttf_font("resources/Monaco.ttf").await.unwrap();

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();
        let fps = get_fps();
        
        field.update(dt);
        field.draw(&mut img);

        texture.update(&img);
        draw_texture_ex(texture, 0.0, 0.0, WHITE, DrawTextureParams{dest_size: Some(Vec2::new(screen_width(), screen_height())), ..Default::default()});

        let fps_text = format!("FPS: {}", fps);
        draw_text_ex(&fps_text, 10.0, 22.0, TextParams{font: font, font_size: 16u16, color: Color::new(1.0, 1.0, 1.0, 0.3), ..Default::default()});

        let n_agents_txt = format!("Agents: {}", n_agents);
        draw_text_ex(&n_agents_txt, 10.0, 42.0, TextParams{font: font, font_size: 16u16, color: Color::new(1.0, 1.0, 1.0, 0.3), ..Default::default()});

        let res_text = format!("Resolution: {}x{}", texture.width(), texture.height());
        draw_text_ex(&res_text, 10.0, 62.0, TextParams{font: font, font_size: 16u16, color: Color::new(1.0, 1.0, 1.0, 0.3), ..Default::default()});

        next_frame().await
    }
}
