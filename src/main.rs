use macroquad::prelude::*;

use std::cmp::PartialEq;

use macroquad::{
    experimental::{
        collections::storage,
        coroutines::start_coroutine,
        scene::{self, Handle},
    },
    ui,
};


mod resources;
use resources::Resources;
mod player;
use player::Player;

type Matrix<'a> = Vec<Vec<Cells>>;

enum NPC {
    Dealer,
    Pilgrim,
    Death
}
#[derive(PartialEq, Clone, Copy)]
enum Cells {
    Nothing,
    Start,
    Enemy,
    Exit,
    // NPC(NPC),
    Weapon,
    Gold,
    Heal,
    Border
    // StatsUp,
}

fn decode_token(token: &str) -> Cells{
    match token {
        "0" => Cells::Exit,
        "1" => Cells::Start,
        "2" => Cells::Nothing,
        "3" => Cells::Enemy,
        "4" => Cells::Gold,
        "5" => Cells::Weapon,
        "6" => Cells::Heal,
        // "7" => Cells::StatsUp,
        _ => panic!("invalid token")
    }
}

// fn get_cell_content(cell: Cells) {
//     let resources = storage::get::<Resources>();
//     match cell {
//         // Cells::Gold => 
//         // Cells::Enemy => (),
//         // Cells::Weapon => (r)
//     }
// }

fn get_matrix3x3<'a>(matrix: Matrix, x: usize, y: usize) -> Matrix{
    let mut map: Matrix = vec![vec![Cells::Border;3], vec![Cells::Border, matrix[y][x], Cells::Border], vec![Cells::Border;3] ];
    let max_x = matrix[y].len() -1;
    let max_y = matrix.len() -1;
    if y != 0 {
        if x != 0{
            map[0][0] = matrix[y-1][x-1];
        }
        if x != max_x{
            map[0][2] = matrix[y-1][x+1]
        }
        map[0][1] = matrix[y-1][x];
    }
    if y != max_y{
        if x != 0{
            map[2][0] = matrix[y+1][x-1];
        }
        if x != max_x{
            map[2][2] = matrix[y+1][x+1];
        }
        map[2][1] = matrix[y+1][x];
    }
    if x != 0 {
        map[1][0] = matrix[y][x-1];
    }
    
    if x != max_x{
        map[1][2] = matrix[y][x+1];
    }
    map
}

fn load_matrix<'a>(content: &'a str) -> Result<(Matrix, usize, usize), String>{
    let mut lst: Matrix = Vec::new();
    let mut y = 0;
    let (mut y_r, mut x) = (None, None);
    for line in content.trim().split('\n') {
        let l: Vec<Cells> = line.split(';').map(|el| decode_token(el.trim())).collect();
        if l.contains(&Cells::Nothing){ 
            y_r = Some(y);
            x = l.iter().position(|&r| r == Cells::Start);
        } //.any(|&i| i=="0")

        lst.push(l);
        y += 1;
    }
    if y_r == None {
        return Err(String::from("Start position not defined"));
    }
    Ok((lst, x.unwrap(), y_r.unwrap()))
}
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

    scene::add_node(Player::new(vec2(w, h)));
    let offset_x = screen_width()  / 3.;
    let offset_y = screen_height() / 3.;
    let line_size = 5.;

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
        }
        // draw_text(&format!("y = {}, x = {}, el = {}",y,x,level_map[y][x]), 140., 140., 40., BLACK);
        next_frame().await;
    }

}
