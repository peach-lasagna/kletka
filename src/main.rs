use macroquad::prelude::*;

use std::collections::LinkedList;
use std::fs;

const SQUARES: i16 = 16;
type Matrix<'a> = Vec<Vec<&'a str>>;
type Point = (i16, i16);

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    dir: Point,
}

fn load_matrix<'a>(content: String) -> Result<(Matrix<'a>, Option<usize>, Option<usize>), String>{
    let mut lst: Matrix<'a> = Vec::new();
    let mut y = 0;
    let (mut y_r, mut x) = (None, None);
    for line in content.split('\n') {
        let l: Vec<&str> = line.split(';').map(|el| el.trim()).collect();
        if l.contains(&"0"){ 
            y_r = y;
            x = l.iter().position(|&r| r == "0").unwrap();
        } //.any(|&i| i=="0") 

        lst.push(l);
        y += 1;
    }
    if y_r == None {
        return Err(String::from("Start position not defined"));
    }
    Ok((lst, x, y_r))
}

#[macroquad::main("Kletka")]
async fn main() {
    let l1 = load_string("assets/levels/l1").await.expect("СРОЧНО НАХУЙ");
    let (level_map, mut  x, mut  y) = match load_matrix(l1){
        Ok(v) => v,
        Err(e) => panic!(e)
    };
    
    loop {
        clear_background(WHITE);
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
        draw_text(&format!("y = {}, x = {}, el = {}",y,x,level_map[y][x]), 140., 140., 40., BLACK);
        next_frame().await;
    }

}
//     let mut snake = Snake {
//         head: (0, 0),
//         dir: (1, 0),
//         body: LinkedList::new(),
//     };
//     let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
//     let mut score = 0;
//     let mut speed = 0.3;
//     let mut last_update = get_time();
//     let mut game_over = false;

//     let up = (0, -1);
//     let down = (0, 1);
//     let right = (1, 0);
//     let left = (-1, 0);

//     loop {
//         if !game_over {
//             if is_key_down(KeyCode::Right) && snake.dir != left {
//                 snake.dir = right;
//             } else if is_key_down(KeyCode::Left) && snake.dir != right {
//                 snake.dir = left;
//             } else if is_key_down(KeyCode::Up) && snake.dir != down {
//                 snake.dir = up;
//             } else if is_key_down(KeyCode::Down) && snake.dir != up {
//                 snake.dir = down;
//             }

//             if get_time() - last_update > speed {
//                 last_update = get_time();
//                 snake.body.push_front(snake.head);
//                 snake.head = (snake.head.0 + snake.dir.0, snake.head.1 + snake.dir.1);
//                 if snake.head == fruit {
//                     fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
//                     score += 100;
//                     speed *= 0.9;
//                 } else {
//                     snake.body.pop_back();
//                 }
//                 if snake.head.0 < 0
//                     || snake.head.1 < 0
//                     || snake.head.0 >= SQUARES
//                     || snake.head.1 >= SQUARES
//                 {
//                     game_over = true;
//                 }
//                 for (x, y) in &snake.body {
//                     if *x == snake.head.0 && *y == snake.head.1 {
//                         game_over = true;
//                     }
//                 }
//             }
//         }
//         if !game_over {
//             clear_background(LIGHTGRAY);

//             let game_size = screen_width().min(screen_height());
//             let offset_x = (screen_width() - game_size) / 2. + 10.;
//             let offset_y = (screen_height() - game_size) / 2. + 10.;
//             let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

//             draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

// //             for i in 1..SQUARES {
// //                 draw_line(
// //                     offset_x,
// //                     offset_y + sq_size * i as f32,
// //                     screen_width() - offset_x,
// //                     offset_y + sq_size * i as f32,
// //                     2.,
// //                     LIGHTGRAY,
// //                 );
// //             }

// //             for i in 1..SQUARES {
// //                 draw_line(
// //                     offset_x + sq_size * i as f32,
// //                     offset_y,
// //                     offset_x + sq_size * i as f32,
// //                     screen_height() - offset_y,
// //                     2.,
// //                     LIGHTGRAY,
// //                 );
// //             }

//             draw_rectangle(
//                 offset_x + snake.head.0 as f32 * sq_size,
//                 offset_y + snake.head.1 as f32 * sq_size,
//                 sq_size,
//                 sq_size,
//                 DARKGREEN,
//             );

//             // for (x, y) in &snake.body {
//             //     draw_rectangle(
//             //         offset_x + *x as f32 * sq_size,
//             //         offset_y + *y as f32 * sq_size,
//             //         sq_size,
//             //         sq_size,
//             //         LIME,
//             //     );
//             // }

//             // draw_rectangle(
//             //     offset_x + fruit.0 as f32 * sq_size,
//             //     offset_y + fruit.1 as f32 * sq_size,
//             //     sq_size,
//             //     sq_size,
//             //     GOLD,
//             // );

//         }// else {
//         //     if is_key_down(KeyCode::Enter) {
//         //         snake = Snake {
//         //             head: (0, 0),
//         //             dir: (1, 0),
//         //             body: LinkedList::new(),
//         //         };
//         //         fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
//         //         score = 0;
//         //         speed = 0.3;
//         //         last_update = get_time();
//         //         game_over = false;
//         //     }
//         // }
//         next_frame().await
//     }
// }

