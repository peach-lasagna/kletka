use crate::player::Player;
use macroquad::prelude::*;


pub trait PlayerInteraction {
    fn update_player(&self, player: &mut Player);
    fn draw(&self, x: f32, y: f32);
}
