use crate::player::Player;
use macroquad::prelude::*;


pub trait PlayerInteraction {
    fn update_player(&self, player: &mut Player);
    fn texture();
    fn draw(&self, x: f32, y: f32){
        draw_texture(
            self.texture(),
            x - self.texture().width()  / 2.,
            y - self.texture().height() / 2.,
            BLACK
        )
    }
}
