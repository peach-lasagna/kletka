use crate::utils::PlayerInteraction ;
use macroquad::experimental::collections::storage;
use crate::player::Player;
use libmacro::BaseObject;
use crate::resources::Resources;
use macroquad::prelude::Texture2D;


#[BaseObject]
pub struct Gold{}

impl Gold {
    pub fn new() -> Self{
        let resources = storage::get::<Resources>();
        let texture = resources.gold;
        Self { texture }
    }
}

impl PlayerInteraction for Gold{
    fn update_player(&self, player: &mut Player){
        player.items.gold += 1;
    }
}

