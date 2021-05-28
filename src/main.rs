mod macroses;
mod player;
mod resources;
mod map;
mod structs;
mod interaction;
mod gold;

use macroquad::prelude::*;


use macroquad::{
    experimental::{
        collections::storage,
        coroutines::start_coroutine,
        scene::{self, Handle},
    },
    ui,
};


use resources::Resources;
use player::Player;
use map::{get_cell_content, load_matrix};


fn conf() -> Conf{
    Conf {
        window_title: String::from("kletka"),
        window_width: 1260,
        window_height: 768,
        fullscreen: true,
        ..Default::default()
}}


#[macroquad::main(conf)]
async fn main() {
    let w = screen_width() / 2.0;
    let h = screen_height() / 2.0;
    let l1 = load_string("assets/levels/l1").await.expect("нету такого");
    
    let resources_loading = start_coroutine(async move {
        let resources = Resources::new().await.unwrap();
        storage::store(resources);
    });

    while resources_loading.is_done() == false {
        clear_background(BLACK);
        draw_text(
            &format!(
                "Loading resources {}",
                ".".repeat(((get_time() * 2.0) as usize) % 4)
            ),
            screen_width() / 2.0 - 160.0,
            screen_height() / 2.0,
            40.,
            WHITE,
        );

        next_frame().await;
    }
    let (mut level_map, mut  x, mut  y) = match load_matrix(&*l1){
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    let mut player = Player::new(vec2(w, h));
    scene::add_node(&mut player);

    let offset_x = screen_width()  / 3.;
    let offset_y = screen_height() / 3.;
    let line_size = 3.;

    loop {
        clear_background(WHITE);
        for i in 1..3{
            draw_line(
                offset_x * i as f32,
                0.,
                offset_x * i as f32,
                screen_height(),
                line_size,
                LIGHTGRAY
            );
            draw_line(
                0.,
                offset_y * i as f32,
                screen_width(),
                offset_y * i as f32,
                line_size,
                LIGHTGRAY
            );
        }
        
        if is_key_down(KeyCode::Q){
            break;
        }
        if is_key_down(KeyCode::D){
            if x != level_map[y].len() - 1{
                x += 1;
            }
        } else if is_key_down(KeyCode::A) {
            if x != 0{
                x -= 1;
            }
        } else if is_key_down(KeyCode::W) {
            if y != 0{
                y -= 1;
            }
        } else if is_key_down(KeyCode::S) {
            if y != level_map.len()-1 {
                y += 1;
            }
        } else {
            continue;
        }
        let cell_obj = get_cell_content(level_map[y][x]);
        player.absorb(cell_obj);
        // draw_text(&format!("y = {}, x = {}, el = {}",y,x,level_map[y][x]), 140., 140., 40., BLACK);
        next_frame().await;
    }

}
