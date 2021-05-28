use crate::interaction::PlayerInteraction;
use crate::structs::{Cells};
use crate::resources::Resources;
use macroquad::experimental::collections::storage;
use crate::gold::Gold;



type Matrix<'a> = Vec<Vec<Cells>>;

pub fn decode_token(token: &str) -> Cells{
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

pub fn get_cell_content(cell: Cells) -> impl PlayerInteraction {
    let resources = storage::get::<Resources>();
    let obj = match cell {
        Cells::Gold => Gold::new(),
        // Cells::Enemy => (),
        // Cells::Weapon => (r)
    };
    obj
}

pub fn get_matrix3x3<'a>(matrix: Matrix, x: usize, y: usize) -> Matrix{
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

pub fn load_matrix<'a>(content: &'a str) -> Result<(Matrix, usize, usize), String>{
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

