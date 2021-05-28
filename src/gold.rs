use crate::interaction::PlayerInteraction ;
// use macroquad::experimental::collections::storage;
use crate::player::Player;
use libmacro::{BaseObject, DefaultDraw};
// use crate::resources::Resources;
// use macroquad::prelude::Texture2D;


#[BaseObject]
pub struct Gold{}

impl Gold {
    pub fn new() -> Self{
        Self { texture: Self::get_texture() }
    }
}


#[DefaultDraw]
impl PlayerInteraction for Gold{
    fn update_player(&self, player: &mut Player){
        player.items.gold += 1;
    }
}

